FROM rust

RUN USER=root cargo new --bin client
WORKDIR /client

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -r src/

COPY src/ src/

RUN rm ./target/release/deps/binary_tree*
RUN cargo install --path .

CMD ["client"]
