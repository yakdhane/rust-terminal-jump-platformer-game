Sure, here's the Dockerfile code:

```
# Base image
FROM rust:latest

# Set working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Set the entrypoint
CMD ["./target/release/jump-platformer"]
```