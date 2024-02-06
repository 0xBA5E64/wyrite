# 🗒️🐉 wyrite

Chronicling my long-running quest of figuring out how to build a functioning blog.

See [`chronicle.md`](docs/chronicle.md)


# Development

You'll want [`sqlx-cli`](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) to manage the database:
```bash
$ cargo install sqlx-cli
$ echo "DATABASE_URL=sqlite://db.sqlite" > .env
$ sqlx database create
$ sqlx migrate run
```
