FROM rust:1.33-slim as builder

WORKDIR /usr/src/myapp

COPY . .

RUN apt update && apt install -y libssl-dev pkg-config
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.9

RUN mkdir -p /app

WORKDIR /app

COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/uptime-probe .

CMD ["/app/uptime-probe"]
