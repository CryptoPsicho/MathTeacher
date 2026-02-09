# Deployment (Docker Compose)

This guide shows how to run the backend and frontend on an Ubuntu 24 VM so the app is reachable from your home network. The frontend is served on port 4000 as requested.

## Prerequisites

- Docker Engine and Docker Compose v2
- Git

On Ubuntu 24:

```bash
sudo apt update
sudo apt install -y docker.io docker-compose-plugin git
sudo usermod -aG docker $USER
newgrp docker
```

## Quick start

1. Clone the repo on the VM.
2. Add the Docker files below.
3. Start the stack with Docker Compose.
4. Open the web UI from any device on your home network.

## Files to add

Create these files in the repo root.

### docker-compose.yml

```yaml
services:
  backend:
    build:
      context: .
      dockerfile: server/Dockerfile
    ports:
      - "4001:4001"
    restart: unless-stopped

  frontend:
    build:
      context: .
      dockerfile: web/Dockerfile
    ports:
      - "4000:4000"
    depends_on:
      - backend
    restart: unless-stopped
```

### server/Dockerfile

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build -p math_teacher_server --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/math_teacher_server /usr/local/bin/math_teacher_server
EXPOSE 4001
CMD ["/usr/local/bin/math_teacher_server"]
```

### web/Dockerfile

```dockerfile
FROM rust:1.82-slim
WORKDIR /app
RUN cargo install dioxus-cli --version 0.7.0
COPY . .
EXPOSE 4000
CMD ["dx", "serve", "-p", "math_teacher_web", "-a", "0.0.0.0", "-p", "4000"]
```

## Run

```bash
docker compose up -d --build
```

Check status:

```bash
docker compose ps
```

View logs:

```bash
docker compose logs -f
```

## Access from your home network

1. Find the VM IP:

```bash
hostname -I
```

2. Open the web UI from another device:

```
http://<vm-ip>:4000
```

## Important notes

- The backend currently binds to `127.0.0.1:4000` in `server/src/main.rs`. For Docker or LAN access, change it to `0.0.0.0:4001` before building the image.
- The frontend calls the backend at `http://127.0.0.1:4000` in `web/src/main.rs`. For LAN access with this layout, update it to `http://<vm-ip>:4001` or a reverse-proxy URL.
- If you use `ufw`, allow the ports:

```bash
sudo ufw allow 4000/tcp
sudo ufw allow 4001/tcp
```

## Stop

```bash
docker compose down
```
