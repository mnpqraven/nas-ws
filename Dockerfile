# BACKUP
# FROM rust:1.67
# WORKDIR /usr/src/nas-ws
# COPY . .
# RUN sudo apt install -y protobuf-compiler libprotobuf-dev && which protoc
# RUN PROTOC=/usr/bin/protoc cargo install --path . --bin nas-ws
# CMD ["nas-ws"]
# EXPOSE 5005

FROM debian:bullseye as builder
WORKDIR /usr/src/nas-ws
RUN apt-get update && apt-get upgrade && apt-get install -y protobuf-compiler libprotobuf-dev curl build-essential pkg-config libssl-dev sqlite3
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
COPY . .

RUN . "$HOME/.cargo/env" && PROTOC=/usr/bin/protoc cargo install --path . --bin nas-ws

FROM rust:bullseye
COPY --from=builder /root/.cargo/bin/nas-ws /usr/local/bin/nas-ws

CMD ["nas-ws"]
EXPOSE 5005