FROM rust:latest

WORKDIR /usr/src/helium

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/helium"]