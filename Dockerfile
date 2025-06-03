# syntax=docker/dockerfile:1

FROM rust:1.87-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rustcors /usr/local/bin/rustcors
# Set default port, can be overridden at runtime
ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rustcors"]

# FROM rust:1.87-alpine3.21 as builder
# WORKDIR /app
# COPY . .
# RUN cargo build --release

# FROM alpine:3.21
# WORKDIR /app
# COPY --from=builder /app/target/release/rustcors /usr/local/bin/rustcors
# # Set default port, can be overridden at runtime
# ENV PORT=8080
# EXPOSE 8080
# ENTRYPOINT ["/usr/local/bin/rustcors"]