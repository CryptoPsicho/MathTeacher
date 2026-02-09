FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build -p math_teacher_server --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/math_teacher_server /usr/local/bin/math_teacher_server
EXPOSE 4001
CMD ["/usr/local/bin/math_teacher_server"]
