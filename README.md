![Wyrite](assets/logo.png)

Chronicling my long-running quest of figuring out how to build a functioning blog, evolved as a low-level reboot of [dyablog](https://github.com/0xBA5E64/dyablog). Currently evaluating `axum`+`sqlx` w. Postgres.

See [`chronicle.md`](docs/chronicle.md)

## todo:
 - [ ] server-side templated frontend
 - [ ] Auth? Passkeys maybe?
 - [X] Serve files from ~~`static/`~~ `assets/`
 - [ ] remove is_published flag, rename date_published to published, check if null for unpublished
 - [ ] (?) Rewrite slug function in python or some other language in the database
 - [ ] Authenticate: new posts, delete posts, publish posts
 - [ ] Fix Slugs getting new superflous suffix when publishing
