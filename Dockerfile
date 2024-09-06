# Use the official Ubuntu base image
FROM ubuntu:24.10

# Update package list and install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y