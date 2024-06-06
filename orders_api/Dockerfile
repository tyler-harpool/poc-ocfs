# Use the official Rust image as a base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create a new stage with a smaller base image
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/orders_api ./

# Set the entrypoint to the binary
ENTRYPOINT ["./orders_api"]

# Expose the port that the application will run on
EXPOSE 3000
