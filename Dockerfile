FROM rust:1.40 as builder
WORKDIR /usr/src/rustapp
COPY . .
RUN rustup override set stable; \
    cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/hello /usr/local/bin/rustapp
CMD ["rustapp"]
EXPOSE 8000
