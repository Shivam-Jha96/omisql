# Build stage
FROM rust:latest AS builder

WORKDIR /usr/src/omnisql
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install any required dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/omnisql/target/release/omnisql /usr/local/bin/omnisql

# Set the entrypoint to the omnisql binary
ENTRYPOINT ["omnisql"]
