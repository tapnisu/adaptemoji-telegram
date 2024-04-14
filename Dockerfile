FROM rust:1.77 as builder
LABEL authors="tapnisu"

WORKDIR /usr/src/adaptemoji-telegram
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runner

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY --from=builder /usr/src/adaptemoji-telegram/target/release/adaptemoji-telegram /usr/local/bin/adaptemoji-telegram

CMD ["adaptemoji-telegram"]
