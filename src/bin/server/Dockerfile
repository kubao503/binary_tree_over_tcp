FROM rust

RUN cargo new --bin server
WORKDIR /server

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -r src/

COPY src/ src/

RUN rm ./target/release/deps/binary_tree*
RUN cargo install --path . --bin server

CMD ["server"]
