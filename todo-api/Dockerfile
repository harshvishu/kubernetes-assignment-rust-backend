# Build stage
FROM rust:1.69 AS build

WORKDIR /usr/src/todo-api
COPY . .

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install libpq5 -y

# Build application
RUN cargo install --path .

# Final stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y openssl ca-certificates libpq-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin/
COPY --from=build /usr/local/cargo/bin/todo-api .

WORKDIR /app

CMD ["todo-api"]