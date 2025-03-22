FROM lukemathwalker/cargo-chef:latest-rust-1 AS base
WORKDIR /app

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && \
  apt-get install -y openssl libssl-dev gcc pkg-config
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build -p shell --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && \
  apt-get install -y --no-install-recommends openssl libssl-dev ca-certificates \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/shell .

ENTRYPOINT [ "/app/shell" ]