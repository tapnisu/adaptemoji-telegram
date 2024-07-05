FROM rust:1.79-alpine3.20 as builder
LABEL authors="tapnisu"

WORKDIR /usr/src/adaptemoji-telegram

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache alpine-sdk libressl-dev

COPY . .
RUN cargo build --release

FROM alpine:3.20 as runner

RUN apk update \
    && apk upgrade --available \
    && apk add --no-cache ca-certificates \
    && update-ca-certificates

COPY --from=builder /usr/src/adaptemoji-telegram/target/release/adaptemoji-telegram /usr/local/bin/adaptemoji-telegram

CMD ["adaptemoji-telegram"]
