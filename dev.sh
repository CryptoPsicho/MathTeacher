#!/usr/bin/env bash
set -euo pipefail

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "${SERVER_PID}" 2>/dev/null || true
  fi
}

trap cleanup EXIT

cargo run -p math_teacher_server &
SERVER_PID=$!

dx serve -p math_teacher_web
