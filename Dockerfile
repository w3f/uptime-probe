FROM clux/muslrust:nightly as builder

WORKDIR /build
ADD . ./
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.13

RUN mkdir -p /app

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/uptime-probe .

CMD ["/app/uptime-probe"]
