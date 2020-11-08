FROM rust:1.47.0-buster as builder

ADD . /src
WORKDIR /src

RUN apt-get update && \
    cargo build --verbose --release && \
    cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/gitlab-pr-bot  /usr/bin

RUN apt update && apt install -y libssl1.1 ca-certificates
CMD gitlab-pr-bot
