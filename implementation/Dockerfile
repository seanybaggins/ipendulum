# Offical base image for Rust
FROM rust:1.49.0

# Install tools that require root privlages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libusb-1.0-0-dev \
    libftdi1-dev \
    gdb \
    openocd \
    gdb-multiarch

# Setting the user to non-root privlages
RUN useradd --create-home --shell /bin/bash rustacean
USER rustacean

# Installing tools in user space
RUN rustup target add thumbv7em-none-eabihf
RUN cargo install cargo-embed --vers 0.10.1
RUN echo 'alias test="cargo test --target x86_64-unknown-linux-gnu"' >> ~/.bashrc