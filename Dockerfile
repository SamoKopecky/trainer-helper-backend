FROM rust:1.84.0-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --bins

FROM rust:1.84.0-bookworm AS runner

RUN apt update -y && apt install -y netcat-traditional

WORKDIR /app

COPY --from=builder /app/target/release/app /app
COPY wait-for.sh /app/wait-for.sh

EXPOSE 2001
CMD [ "/app/backend" ]

