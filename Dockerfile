FROM rust:latest

COPY . .

RUN cargo install wasm-pack

RUN wasm-pack build
