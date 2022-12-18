# This relies on the base image created by Dockerfile.base

FROM vgwidt/sumi-base:latest

WORKDIR /usr/src/sumi

# Copy updated files
COPY . .

# Build frontend and run backend
RUN trunk build -d dist ./frontend/index.html --release
RUN cargo build --manifest-path ./backend/Cargo.toml --release
# move binary otherwise cargo clean will wipe it
RUN mv ./target/release/backend ./sumi
RUN cargo clean
RUN cargo cache -a