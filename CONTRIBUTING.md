# Contribution guide

## Dependencies

Create a _virtualenv_ with Python 3.9 and install the dependencies with `pip`, e.g.:

```console
$ python -m venv .venv
$ source .venv/bin/activate
$ pip install -r requirements.txt
```

You'll also need a PostgreSQL URI.

## Setup

Create the environment variables as in `.env.sample` or use `createnv` to guide you. Once you're done, run the database migrations:

```console
$ createnv
$ python manage.py migrate
```

## Running

You can run the bot continuously, mainly for development with:

```console
$ python manage.py bot
```

If using it as a webhook, it's better to run Django according to production standards (e.g. with ASGI).

## Tests

```console
$ pip install -r requirements-dev.txt 
$ python manage.py test
$ black . --check
$ isort --profile black **/*.py
$ flake8 **/*.py --max-line-length 88 --ignore E203
```
