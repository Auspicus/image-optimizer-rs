FROM rust:1.54 as builder
WORKDIR /usr/src/image-optimizer-rs
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/image-optimizer-rs /usr/local/bin/image-optimizer-rs
RUN apt-get update -y; \
    apt-get install ca-certificates -y;