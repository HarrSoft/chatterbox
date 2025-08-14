FROM rust:1.89-alpine3.22 AS builder
WORKDIR /build
RUN apk add openssl-dev
COPY . .
RUN cargo install --locked --root /build/install --path .

FROM alpine:3.22
WORKDIR /app
COPY --from=builder /build/install .
CMD ["/app/bin/chatterbox"]
