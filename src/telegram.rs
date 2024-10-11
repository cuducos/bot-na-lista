use crate::models::List;
use crate::{chat::Chat, db};
use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;
use teloxide::{
    dispatching::{DefaultKey, UpdateFilterExt},
    dptree,
    prelude::{Bot, Dispatcher, LoggingErrorHandler, Message, Request, Requester},
    respond,
    types::{ChatId, MessageKind, Update},
    update_listeners::webhooks,
    RequestError,
};
use url::Url;

const DEFAULT_HOST_IP: [u8; 4] = [0, 0, 0, 0];
const HELP: &str = "Send me a message.

If the message is a number, I will delete the item from the list that has that number.

If the message is not a number, I will add the item to the list.

Use /view to see the list.

Lists with no activity for more than one year are automatically deleted.";

enum Response {
    Text(String),
    List(List),
}

async fn send_response(bot: &Bot, chat_id: ChatId, response: Response) {
    let bot = bot.clone();
    let (text, label) = match response {
        Response::Text(txt) => (txt, "Message"),
        Response::List(list) => (format!("{}", list), "List"),
    };
    match bot.send_message(chat_id, text).send().await {
        Ok(_) => log::debug!("[List#{}] {} sent successfully", chat_id, label),
        Err(e) => log::error!(
            "[List#{}] Error sending {}: {:?}",
            chat_id,
            label.to_lowercase(),
            e
        ),
    };
}

async fn process_message(pool: Pool<ConnectionManager<PgConnection>>, bot: &Bot, msg: &Message) {
    if let MessageKind::Common(_) = &msg.kind {
        if let Some(txt) = msg.text() {
            if txt == "/help" || txt == "/start" {
                return send_response(bot, msg.chat.id, Response::Text(HELP.to_string())).await;
            }
            match Chat::new(msg.chat.id.0, &pool) {
                Ok(mut chat) => {
                    let reply = if txt == "/view" {
                        chat.list()
                    } else {
                        chat.process_input(txt)
                    };
                    match reply {
                        Ok(list) => send_response(bot, msg.chat.id, Response::List(list)).await,
                        Err(e) => log::error!(
                            "[List#{}] Error processing input `{}`: {}",
                            chat.chat_id,
                            txt,
                            e
                        ),
                    }
                }
                Err(e) => log::error!("[List#{}] Error handling chat: {}", msg.chat.id.0, e),
            }
        }
    }
}

async fn webhook(
    bot: Bot,
    port: u16,
    host: String,
    mut dispatcher: Dispatcher<Bot, RequestError, DefaultKey>,
) -> Result<()> {
    let url = Url::parse(format!("https://{host}/webhook").as_str())?;
    let opts = webhooks::Options::new((DEFAULT_HOST_IP, port).into(), url.clone())
        .max_connections(db::max_connections() as u8);
    dispatcher
        .dispatch_with_listener(
            webhooks::axum(bot, opts).await?,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
    Ok(())
}

pub async fn run(pool: Pool<ConnectionManager<PgConnection>>) -> Result<()> {
    let port = env::var("PORT").ok().and_then(|p| p.parse::<u16>().ok());
    let host = env::var("HOST").ok();
    let bot = Bot::from_env();
    let handler = dptree::entry().branch(Update::filter_message().endpoint(
        |bot: Bot, pool: Pool<ConnectionManager<PgConnection>>, msg: Message| async move {
            process_message(pool.clone(), &bot, &msg).await;
            respond(())
        },
    ));
    let mut dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build();
    match (port, host) {
        (Some(p), Some(h)) => {
            log::info!("Starting bot in webhook mode...");
            webhook(bot, p, h, dispatcher).await?;
        }
        _ => {
            log::info!("Starting bot in polling mode...");
            dispatcher.dispatch().await;
        }
    }
    Ok(())
}
