FROM ubuntu:18.04

RUN apt update && \
    apt install -y \
      build-essential \
      curl \
      libssl-dev

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

WORKDIR /build
COPY entrypoint.sh /root/entrypoint.sh
ENTRYPOINT /root/entrypoint.sh
