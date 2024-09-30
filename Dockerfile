FROM rust:1.80-bullseye as builder
WORKDIR /app
ARG DB_URL

COPY . .
ENV SQLX_OFFLINE=true
# RUN apt-get update && apt-get install -y pkg-config libssl-dev

ENV DATABASE_URL=${DB_URL}
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
RUN cargo sqlx prepare
RUN cargo build --release

FROM debian:bullseye-slim

# RUN apt-get update && apt-get install -y \
#     libssl1.1 \
#     ca-certificates \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

# RUN apt-get update && apt install -y openssl

COPY --from=builder /app/target/release/rust-ddd-oauth-casbin /app/.
COPY --from=builder /app/migrations/. /app/migrations/
COPY --from=builder /app/etc/. /app/etc/
COPY --from=builder /app/.env /app/.env

# CMD ["./app/rust-ddd-oauth-casbin"]
