# ==============================
# STAGE 1: Base Development Image
# ==============================
FROM rust:latest AS dev

ARG DEBIAN_FRONTEND=noninteractive

# Instal dependencies umum dan library MySQL + SSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    default-libmysqlclient-dev \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Instal cargo-watch agar bisa hot reload
RUN cargo install cargo-watch

# Set direktori kerja
WORKDIR /usr/src/app

# Copy Cargo.toml dan Cargo.lock dulu untuk cache dependencies
COPY Cargo.toml Cargo.lock ./

# Buat dummy main.rs agar cargo build bisa caching dependency layer
RUN mkdir src && echo "fn main() { println!(\"building cache...\"); }" > src/main.rs

# Prebuild dependencies (lebih cepat saat container start)
RUN cargo build && rm -rf src

# Copy seluruh kode proyek ke container
COPY . .

# Default command di stage ini hanya untuk placeholder
CMD ["cargo", "watch", "-q", "-x", "run"]

# ==============================
# STAGE 2: Release Image (Production)
# ==============================
FROM debian:bookworm-slim AS prod

RUN apt-get update && apt-get install -y \
    libssl3 \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=dev /usr/src/app/target/release/todolist .

EXPOSE 8180
CMD ["./todolist"]
