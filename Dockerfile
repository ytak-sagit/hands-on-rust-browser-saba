# syntax=docker/dockerfile:1

FROM mcr.microsoft.com/devcontainers/rust:latest

RUN <<EOF
# Setup rustup
rustup show

# Update apt packages
apt update

# Install QEMU
apt install -y qemu-system

# Clean up
apt autoremove -y
apt clean -y
rm -rf /var/lib/apt/lists/*
EOF
