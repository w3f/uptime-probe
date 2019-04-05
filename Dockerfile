FROM rust:1.33-slim as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release


FROM alpine:3.9

RUN mkdir -p /app

WORKDIR /app

COPY --from=builder /usr/src/myapp/target/release/uptime-probe .

CMD ["/app/uptime-probe"]
