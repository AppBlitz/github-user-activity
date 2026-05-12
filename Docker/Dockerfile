FROM rust:1.95-slim-bullseye as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/github-user-activity /usr/local/bin/github-user-activity
ENTRYPOINT ["/usr/local/bin/github-user-activity"]




