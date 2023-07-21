FROM rust:latest as builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:stable-slim as runner

COPY --from=builder /app/target/release/prophecy-maths .

CMD ["./prophecy-maths"]
