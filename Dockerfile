FROM rust:1.75 AS builder

WORKDIR /usr/get-img

COPY . .

RUN cargo build --release

# multi-stage build to reduce final size
FROM debian:stable-slim

# volume for
VOLUME ["/usr/get-img/download"]

RUN apt update && apt install -y libssl-dev ca-certificates

WORKDIR /usr/get-img

COPY config.toml .

COPY --from=builder /usr/get-img/target/release/get-img /usr/get-img

CMD ["/usr/get-img/get-img"]