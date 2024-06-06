# Use the official Rust image as the build stage
FROM rust:latest as builder

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Use the official Debian image as the runtime stage
FROM debian:latest
# Install required dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# Copy the built binary from the build stage
COPY --from=builder /usr/src/app/target/release/gateway_api /usr/local/bin/gateway_api

# Expose the port the application will run on
EXPOSE 8000

# Set the entrypoint to the built binary
ENTRYPOINT ["gateway_api"]
