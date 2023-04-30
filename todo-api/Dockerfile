FROM rust:1.55.0-slim-bullseye AS builder

WORKDIR /usr/src/todo-api
COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/todo-api /usr/local/bin/todo-api

CMD ["todo-api"]