FROM rust:1.88-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
RUN cargo install dioxus-cli --version 0.7.0 --locked
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "-p", "math_teacher_web", "-a", "0.0.0.0", "-p", "4000"]
