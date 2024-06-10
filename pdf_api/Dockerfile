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
    curl

# Install PDFium
RUN mkdir /pdfium && \
    cd /pdfium && \
    git clone --depth 1 --branch chrome/4867 https://pdfium.googlesource.com/pdfium.git && \
    cd pdfium && \
    mkdir build && \
    cd build && \
    cmake .. -DCMAKE_BUILD_TYPE=Release && \
    make -j$(nproc) && \
    make install

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 3017

# Set the command to run the application
CMD ["./target/release/pdf_api"]
