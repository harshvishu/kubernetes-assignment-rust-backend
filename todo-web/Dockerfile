FROM rust:1.69 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

WORKDIR /usr/local/bin/

COPY --from=builder /usr/local/cargo/bin/todo-web .

EXPOSE 8000

CMD ["./todo-web"]
