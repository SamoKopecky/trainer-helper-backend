FROM rust:1.84.0-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:1.84.0-bookworm AS runner

WORKDIR /app

COPY --from=builder /app/target/release/backend  /app

CMD [ "/app/backend" ]

