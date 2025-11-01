# ==================================
# STAGE 1: Build a-release-binary
# ==================================
FROM rust:latest AS builder

ARG DEBIAN_FRONTEND=noninteractive

# Instal library development untuk KOMPILASI
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    default-libmysqlclient-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# ========== TAMBAHKAN INI ==========
# Instal cargo-watch untuk hot-reloading
RUN cargo install cargo-watch
# ==================================

WORKDIR /usr/src/app
COPY . .

# ========== TAMBAHAN BARU ==========
# Beri tahu SQLx untuk tidak terhubung ke DB saat kompilasi
# Ini akan memperbaiki error "Connection refused"
#ENV SQLX_OFFLINE=true

RUN cargo build --release

# ==================================
# STAGE 2: Build Image Final
# ==================================
FROM debian:bookworm-slim AS runtime

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    libssl3 \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*


WORKDIR /app
COPY --from=builder /usr/src/app/target/release/todolist .

EXPOSE 8180
CMD ["./todolist"]
