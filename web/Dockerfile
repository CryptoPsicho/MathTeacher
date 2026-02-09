FROM rust:1.88-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
 && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -fsSL https://dioxus.dev/install.sh | bash
WORKDIR /app
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "-p", "math_teacher_web", "-a", "0.0.0.0", "-p", "4000"]
