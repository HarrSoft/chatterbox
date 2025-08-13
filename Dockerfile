FROM rust:1.89-alpine3.22 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:3.22
WORKDIR /app
COPY --from=builder /build/target .
CMD ["/app/target/bin/chatterbox"]
