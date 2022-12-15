FROM rust:latest

COPY . .

RUN cargo build --bin prophecy-maths --release
