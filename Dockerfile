FROM rust

RUN USER=root cargo new --bin tcp_server
WORKDIR /tcp_server

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -r src/

COPY src/ src/

RUN rm ./target/release/deps/tcp_server*
RUN cargo install --path .

CMD ["tcp_server"]
