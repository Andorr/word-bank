FROM ekidd/rust-musl-builder:stable as builder
ENV PKG_CONFIG_ALLOW_CROSS=1

# Cache dependencies
RUN USER=root cargo new --lib lib
# COPY ./lib/Cargo.lock ./lib/Cargo.lock
COPY ./lib/Cargo.toml ./lib/Cargo.toml

RUN USER=root cargo new --bin api
# COPY ./api/Cargo.lock ./api/Cargo.lock
COPY ./api/Cargo.toml ./api/Cargo.toml
RUN cd api && cargo build --release

# Build lib for release
RUN cd lib && rm -rf src/*.rs
COPY ./lib/src ./lib/src
RUN rm ./api/target/x86_64-unknown-linux-musl/release/deps/lib-*
RUN rm ./api/target/x86_64-unknown-linux-musl/release/deps/liblib-*
RUN cd api && cargo build --release

# Build api for release
RUN cd api && rm -rf src/*.rs
COPY ./api/src ./api/src
RUN rm ./api/target/x86_64-unknown-linux-musl/release/deps/api-*
RUN cd api && cargo build --release

FROM alpine:latest

COPY --from=builder /home/rust/src/api/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/api

ENTRYPOINT [ "api" ]