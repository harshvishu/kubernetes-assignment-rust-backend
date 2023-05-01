FROM rust:1.69-slim-bullseye AS builder

WORKDIR /usr/src/todo-web
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/todo-web /usr/local/bin/todo-web

EXPOSE 8000

CMD ["todo-web"]
