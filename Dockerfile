FROM rust:latest as builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM rust:latest as runner

COPY --from=builder /app/target/release/prophecy-maths .

CMD ["./prophecy-maths"]
