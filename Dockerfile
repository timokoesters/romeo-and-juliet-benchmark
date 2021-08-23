# syntax=docker/dockerfile:1
FROM docker.io/library/rust:slim as builder

WORKDIR /app
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY ./src src
RUN cargo build --release

FROM docker.io/library/debian:buster


RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY ./romeo_and_juliet.txt .
COPY --from=builder /app/target/release/rjbench .
ENTRYPOINT ["time", "/app/rjbench"]