FROM rust:1.54 as builder
WORKDIR /usr/src/rustapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rustapp /usr/local/bin/rustapp
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
CMD ["rustapp"]
EXPOSE 8000
