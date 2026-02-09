# Math Teacher

Small Rust workspace with an axum backend and Dioxus 0.7 web UI to generate printable multiplication worksheets.

## Requirements

- Rust toolchain
- Dioxus CLI (`dx`)

## Run

Backend:

```
cargo run -p math_teacher_server
```

Frontend:

```
dx serve -p math_teacher_web
```

Open the web app, select the tables, pick the number of questions (1-30), and press Create. Regenerate keeps the same selections and count.

## Notes

- The PDF opens in a new tab and is sized to A4 with two columns.
- Questions are random multipliers from 0 to 10 for the selected tables.
