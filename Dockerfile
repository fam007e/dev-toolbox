FROM rust:latest

RUN apt-get update && apt-get install -y build-essential

WORKDIR /usr/src/dev-toolbox

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc

COPY . .

RUN cargo build --release