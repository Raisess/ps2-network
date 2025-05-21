FROM rust:1.87-slim-bullseye

RUN apt-get update
RUN apt-get install -y \
      ca-certificates \
      pkg-config \
      libssl-dev \
      libfuse-dev \
      fuse3

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

ENTRYPOINT /usr/src/app/target/release/server
