# Use the official Rust image to build your application
FROM rust:1.75 as builder

WORKDIR /usr/src/azeroth_dock

# Copy your manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Trick to cache dependencies: create a dummy source file to ensure dependencies are cached.
RUN mkdir src && \
    echo "fn main() {println!(\"If you see this, the build process might have gone wrong.\");}" > src/main.rs

# Build only dependencies to cache them
RUN cargo build --release

# Remove the dummy source & target directory. This ensures only actual application binaries are compiled.
RUN rm -rf ./src ./target/release/deps/azeroth_dock*

# Copy your actual source tree
COPY ./src ./src
COPY ./templates ./templates

# Build for release. Dependencies will be reused from cache.
RUN cargo build --release

# Use the Debian slim image for the runtime base
FROM debian:bookworm-slim

# Install SSL certificates and possible SSL libraries
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /azeroth_dock

# Copy the compiled binary and templates directory from the builder stage
COPY --from=builder /usr/src/azeroth_dock/target/release/azeroth_dock .
COPY --from=builder /usr/src/azeroth_dock/templates ./templates

# Ensure the binary is executable
RUN chmod +x azeroth_dock

# Set Rocket to listen on all network interfaces
ENV ROCKET_ADDRESS=0.0.0.0

# This command runs your application
CMD ["./azeroth_dock"]
