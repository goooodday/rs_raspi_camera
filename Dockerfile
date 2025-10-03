# Use an ARM64 Ubuntu base image
FROM --platform=linux/arm64 arm64v8/ubuntu:25.04

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies, including OpenCV
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libopencv-dev \
    libclang-dev \
    clang \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project
# We don't need the rpi4 alias anymore because we are not cross-compiling
CMD ["cargo", "build", "--release"]
