# Build stage
FROM rust:1.69 AS build

WORKDIR /usr/src/todo-web
COPY . .

# Build application
RUN cargo install --path .

# Final stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin/
COPY --from=build /usr/local/cargo/bin/todo-web .

WORKDIR /app

EXPOSE 8000

CMD ["todo-web"]