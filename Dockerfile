# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the Rust project
RUN cargo build --release

# Debugging step: List the contents of the /app directory
RUN ls -la /app

# Debugging step: List the contents of the /target/release directory
RUN ls -la /app/target/release

# Run the executable
ENTRYPOINT ["./app/target/release/fibbot-test"]
