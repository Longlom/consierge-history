# Builder Stage
FROM rust:bookworm as builder

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder ./target/release/consierge-history ./target/release/consierge-history

EXPOSE 5000

CMD ["./target/release/consierge-history"]
