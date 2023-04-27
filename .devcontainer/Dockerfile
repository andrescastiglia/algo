FROM rust:bullseye 

RUN apt-get update -qq && \
    apt-get upgrade -y -qq && \
    apt-get install protobuf-compiler -y -qq && \
    rm -rf /var/lib/apt/lists/* && \
    rustup update nightly && \
    rustup default nightly && \
    rustup component add clippy && \
    rustup component add rustfmt

ENV RUST_LOG=debug
ENV RUST_TEST_THREADS=1