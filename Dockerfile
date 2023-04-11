FROM rust:1.67
WORKDIR /usr/src/nas-ws
COPY . .
RUN cargo install --path .
CMD ["nas-ws"]
EXPOSE 5005
