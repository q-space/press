# Production image -- BB26090904. Multi-stage release build; Dockerfile.dev
# (cargo-watch, untagged rolling image) stays the local-dev image
# (BB26090903) -- this file is a genuinely separate artifact, not a variant
# of it. Pinned to a specific rust:slim tag (not Dockerfile.dev's rolling
# latest) since a production image should be reproducible -- cargo-watch's
# MSRV churn that forced Dockerfile.dev to stay unpinned doesn't apply here
# at all (cargo-watch is a dev-only tool this image never installs).
FROM rust:1.97-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY app/api .
RUN cargo build --release

FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/qspace-press-api ./qspace-press-api
EXPOSE 3001
CMD ["./qspace-press-api"]
