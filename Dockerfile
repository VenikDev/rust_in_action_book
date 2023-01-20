FROM rust:1.31

WORKDIR /usr/src/rust_in_action
COPY . .

RUN cargo install --path .

CMD ["rust_in_action"]