![Wyrite](assets/logo.png)

Chronicling my long-running quest of figuring out how to build a functioning blog, evolved as a low-level reboot of [dyablog](https://github.com/0xBA5E64/dyablog). Currently evaluating `axum`+`sqlx` w. Postgres.

See [`chronicle.md`](docs/chronicle.md)

## todo:
 - [ ] Web: Unify HTTP/HTML response handling, both OK and Error
 - [ ] Better error-management with ThisError
 - [ ] Web: Auth
   - [ ] Implement Authentication middleware
   - [ ] Implement JWT
   - [ ] Protect endpoints for: new posts, (edit posts?) delete posts, publish posts
   - [ ] Investigate passkey option?
   - [ ] API: token system maybe?
 - [ ] Migrate from UUIDv1 to UUIDv7
 - [X] server-side templated frontend
 - [X] Serve files from ~~`static/`~~ `assets/`
 - [X] remove is_published flag, rename date_published to published, check if null for unpublished
 - [ ] (?) Rewrite slug function in python or some other language in the database
 - [X] Fix Slugs getting new superflous suffix when publishing
