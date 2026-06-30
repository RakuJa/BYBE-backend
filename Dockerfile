# Stage 1: Build the Rust project
FROM rust:1-slim-bookworm AS builder

# Set the working directory in the container
WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends python3 curl zlib1g-dev

# Copy the project files into the container
COPY . .

# Build the project with optimizations
RUN cargo install --no-default-features --force cargo-make
RUN cargo make bybe-docker-release

# Stage 2: Create a minimal runtime image
FROM alpine:latest
# Combatibility for glibc based binary with musl
RUN apk add --no-cache gcompat libssl3 ca-certificates

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/bybe .
COPY --from=builder /app/data/bybe_pglite.sql data/
COPY --from=builder /app/data/names.json data/
COPY --from=builder /app/data/nicknames.json data/

ENV SQL_PATH="/app/data/bybe_pglite.sql"
ENV SERVICE_STARTUP_STATE="Clean"
ENV NAMES_PATH="/app/data/names.json"
ENV NICKNAMES_PATH="/app/data/nicknames.json"
ENV BACKEND_URL="https://api.bybe.com"

# Expose the port that your Actix-Web application will listen on
EXPOSE 25566
# Command to run your application when the container starts
ENTRYPOINT ["./bybe"]
