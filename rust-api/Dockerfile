FROM rust

WORKDIR /usr/src/rust-api
COPY . .

RUN cargo install --path .

CMD ["rust-api"]