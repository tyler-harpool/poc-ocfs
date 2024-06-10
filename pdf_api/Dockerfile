# Use the official Rust image as a base
FROM rust:latest

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    tesseract-ocr \
    libleptonica-dev \
    git \
    cmake \
    clang \
    libc++-dev \
    libc++abi-dev \
    build-essential \
    pkg-config \
    libfontconfig1-dev \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libglu1-mesa-dev \
    curl \
    wget \
    unzip




# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release
# Copy PDFium library
COPY pdfium/linux/lib/libpdfium.so /usr/local/lib
RUN ls /usr/local/lib/libpdfium.so
# Expose the port the app runs on
EXPOSE 3017

# Set the command to run the application
CMD ["./target/release/pdf_api"]
