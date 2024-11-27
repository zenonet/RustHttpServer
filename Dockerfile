FROM rust:latest
EXPOSE 80

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

ENTRYPOINT ["SocketStuff"]
