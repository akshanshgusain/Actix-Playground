FROM rust AS builder
COPY . /app
WORKDIR /app
RUN cargo build --release
FROM debian:buster-slim
COPY --from=builder /app/target/release/todo_actix /app/todo_actix
WORKDIR /app
CMD ["./todo_actix"]