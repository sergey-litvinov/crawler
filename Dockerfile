FROM rust:1.50 as builder
WORKDIR /usr/src/crawler
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ENV RUST_LOG="warn,crawler=trace"
ENV APP_TELEGRAM-TOKEN=""
ENV APP_TELEGRAM-CHAT_ID=""
WORKDIR /usr/local/crawler
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/crawler /usr/local/crawler/crawler
COPY --from=builder /usr/src/crawler/config /usr/local/crawler/config
CMD ["./crawler"]