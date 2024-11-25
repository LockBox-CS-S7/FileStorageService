FROM rust:latest AS Build

WORKDIR /app

COPY . .

RUN cargo test
RUN cargo build --release


FROM debian:trixie-slim AS Release

WORKDIR /app

COPY --from=Build /app/target/release/lockbox-fs-service .
EXPOSE 8080

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ["./lockbox-fs-service"]