FROM rust:1.85.0 AS build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=build-env /app/target/release/api /app
COPY --from=build-env /app/target/release/indexer /app
CMD ["/app/api"]
