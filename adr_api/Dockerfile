# Stage 1: Build the application
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin adr_api
WORKDIR /adr_api

# Copy the manifests
COPY Cargo.toml Cargo.lock ./

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree
COPY . .

# Build for release
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /adr_api/target/release/adr_api /usr/local/bin/adr_api

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["adr_api"]
