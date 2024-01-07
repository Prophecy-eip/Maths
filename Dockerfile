FROM rust:latest as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:stable-slim as runner

COPY ai/flask_config.json ai/flask_config.json

RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && update-ca-certificates

COPY --from=builder /app/target/release/prophecy-maths .

CMD ["./prophecy-maths"]
