FROM ekidd/rust-musl-builder as builder

ADD . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.9

RUN mkdir -p /app

WORKDIR /app

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/uptime-probe .

CMD ["/app/uptime-probe"]
