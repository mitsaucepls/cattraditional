FROM rust:1.81-alpine AS builder
WORKDIR /app

RUN USER=root

RUN apk add pkgconfig openssl-dev libc-dev

COPY ./ ./

RUN cargo build --release

FROM alpine:latest
WORKDIR /app

RUN apk update \
    && apk add openssl ca-certificates

EXPOSE 3000

COPY --from=builder /app/target/release/catcomponent /app/catcomponent

CMD ["/app/catcomponent"]
