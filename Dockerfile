FROM rust:1.87-alpine3.21 as build-env
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo test
RUN cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=build-env /app/target/release/rustcors /usr/local/bin/rustcors
ENTRYPOINT ["/usr/local/bin/rustcors"]
