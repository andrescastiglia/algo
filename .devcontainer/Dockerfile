FROM rust:bullseye 

RUN apt-get update -qq && \
    apt-get upgrade -y -qq && \
    apt-get install protobuf-compiler -y -qq && \
    rm -rf /var/lib/apt/lists/* && \
    rustup update stable && \
    rustup component add clippy && \
    rustup component add rustfmt

ENV RUST_LOG=debug
