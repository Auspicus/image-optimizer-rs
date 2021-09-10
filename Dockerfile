FROM rust:1.54 as builder
WORKDIR /usr/src/rustapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/image-optimizer-rs /usr/local/bin/rustapp
RUN apt-get update -y; \
    apt-get install libwebp-dev -y; \
    apt-get install ca-certificates -y;
CMD ["rustapp"]