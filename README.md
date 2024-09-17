# Rust Clean Architecture Boilerplate

This is my boilerplate using rust to make an REST API with some stuff included that i dont want to make it again again again and again LOL :).

## Tech Stacks

- Rust
- Axum
- Postges
- SQLx
- Casbin
- JWT
- Argon2
- Google Oauth 2.0 (without oauth2 crate)
- Redis
- bb8_redis for Async Redis Pool Connection

## IMPORTANT!

You may notice on all my commits if there's some changes on my migrations i didn't add new migration, because earlier the project wasn't stable and still on development that's why i keep changing the existing migration instead.
but after this project is stable and ready to be used i will add new migration if there's any changes regarding the database structure.
and most importantly this project maybe not suitable for your cases or not efficient for you, so i dont care i just use this and hopefully it will be better next time :).
