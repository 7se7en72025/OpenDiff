# Use Rust official image
FROM rust:1.70 as builder

WORKDIR /app

# Copy Cargo files
COPY backend/Cargo.toml Cargo.toml
COPY backend/src src
COPY frontend/public frontend/public

# Build the application
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/opendiff-backend .

# Expose port
EXPOSE 5000

# Run the application
CMD ["./opendiff-backend"]
