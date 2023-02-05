FROM rust:latest as base
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

FROM base as planner
WORKDIR /usr/src/sumi
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as cacher
WORKDIR /usr/src/sumi
COPY --from=planner /usr/src/sumi/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM base as builder
WORKDIR /usr/src/sumi
COPY . .
COPY --from=cacher /usr/src/sumi/target target
RUN trunk build -d dist ./frontend/index.html --release
RUN cargo build --manifest-path ./backend/Cargo.toml --release

FROM rust:latest
WORKDIR /usr/src/sumi
RUN apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
COPY --from=builder /usr/src/sumi/dist ./dist
COPY --from=builder /usr/src/sumi/target/release/sumi-backend ./sumi