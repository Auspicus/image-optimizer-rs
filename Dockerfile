FROM rust:1.54
WORKDIR /usr/src/rustapp
COPY . .
RUN cargo install --path .
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8000"
CMD ["rustapp"]
EXPOSE 8000
