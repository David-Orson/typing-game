FROM rust as builder

WORKDIR /usr/src/rust-api

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/rust-api /usr/local/bin/rust-api

CMD ["rust-api"]