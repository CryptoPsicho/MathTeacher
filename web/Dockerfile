FROM ubuntu:24.04
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    unzip \
 && rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal
RUN /root/.cargo/bin/rustup target add wasm32-unknown-unknown
RUN mkdir -p /root/.cargo/bin && curl -fsSL https://dioxus.dev/install.sh | bash
WORKDIR /app
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "--package", "math_teacher_web", "--addr", "0.0.0.0", "--port", "4000", "--web"]
