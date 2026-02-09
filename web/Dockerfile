FROM ubuntu:24.04
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    unzip \
 && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.cargo/bin:${PATH}"
RUN mkdir -p /root/.cargo/bin && curl -fsSL https://dioxus.dev/install.sh | bash
WORKDIR /app
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "-p", "math_teacher_web", "-p", "4000"]
