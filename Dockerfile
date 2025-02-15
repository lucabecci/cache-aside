FROM rust:1.75 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/cache-aside-rust .

EXPOSE 8080

CMD ["./cache-aside-rust"]
