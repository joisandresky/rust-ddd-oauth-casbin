FROM rust as builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN apt-get update && apt-get install -y pkg-config libssl-dev

ENV DATABASE_URL=postgres://postgres:@host.docker.internal/rust-ddd-oauth-casbin
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
RUN cargo sqlx prepare
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-ddd-oauth-casbin .
COPY --from=builder /app/migrations .
COPY --from=builder /app/etc .
COPY --from=builder /app/.env .

CMD ["./rust-ddd-oauth-casbin"]
