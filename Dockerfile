FROM rust:1.67
EXPOSE 80

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

ENTRYPOINT ["SocketStuff"]
