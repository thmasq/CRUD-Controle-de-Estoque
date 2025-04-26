# Stage 1: Build the application
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev

# Create a new empty shell project
WORKDIR /usr/src/stock-manager

# Copy project files
COPY . .

# Build the project
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/stock-manager/target/release/stock-web-server /usr/local/bin/

# Set working directory
WORKDIR /app

# Expose the application port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=stock_web_server=debug,actix_web=info
ENV DATABASE_URL=postgres://postgres:postgres@db:5432/stockmanager
ENV HOST=0.0.0.0
ENV PORT=8080

# Run the application
CMD ["stock-web-server"]
