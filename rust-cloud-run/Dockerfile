FROM rust:1.87-slim-bookworm as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .

COPY src ./src
COPY templates ./templates

RUN cargo build --release
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3

WORKDIR /app

COPY --from=builder /app/target/release/rust-cloud-run .

EXPOSE 8080

CMD ["./rust-cloud-run"]