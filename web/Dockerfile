FROM rust:1.88-slim
WORKDIR /app
RUN cargo install dioxus-cli --version 0.7.0
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "-p", "math_teacher_web", "-a", "0.0.0.0", "-p", "4000"]
