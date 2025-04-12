FROM rust:1.86 as builder

WORKDIR /usr/src/stock-manager

COPY ./stock-manager .

RUN cargo build --release --bin stock-api-server --bin stock-web-server --bin stock-cli

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/stock-manager/target/release/stock-api-server /app/
COPY --from=builder /usr/src/stock-manager/target/release/stock-web-server /app/
COPY --from=builder /usr/src/stock-manager/target/release/stock-cli /app/

COPY ./stock-manager/init.sql /app/
COPY ./stock-manager/seed.sql /app/
COPY ./stock-manager/seed.sh /app/

RUN chmod +x /app/seed.sh

RUN mkdir -p /app/static

CMD ["./stock-api-server"]
