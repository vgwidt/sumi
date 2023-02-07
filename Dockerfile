FROM rust:latest as base
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

FROM base as cacher
WORKDIR /usr/src/sumi
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./frontend/Cargo.toml ./frontend/Cargo.toml
COPY ./backend/Cargo.toml ./backend/Cargo.toml
COPY ./frontend/.cargo ./frontend/.cargo
RUN mkdir ./backend/src && mkdir ./frontend/src && echo 'fn main() { println!("Dummy"); }' > ./backend/src/main.rs && echo 'fn main() { println!("Dummy"); } ' > ./frontend/src/lib.rs
RUN cargo build --release --manifest-path ./backend/Cargo.toml
WORKDIR /usr/src/sumi/frontend
RUN cargo build --release

FROM base as builder
WORKDIR /usr/src/sumi
COPY --from=cacher /usr/src/sumi/target target
COPY . .
RUN trunk build -d dist ./frontend/index.html --release
RUN cargo build --manifest-path ./backend/Cargo.toml --release

FROM rust:latest
WORKDIR /usr/src/sumi
RUN apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
COPY --from=builder /usr/src/sumi/dist ./dist
COPY --from=builder /usr/src/sumi/target/release/sumi-backend ./sumi