FROM rust:1.67
WORKDIR /usr/src/nas-ws
COPY . .
RUN cargo install --path . --bin nas-ws
CMD ["nas-ws"]
EXPOSE 5005
