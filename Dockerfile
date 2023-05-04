FROM rust:1.69-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/codex_gigas
COPY . ./
RUN cargo build --release

FROM rust:1.69-alpine
WORKDIR /usr/src/codex_gigas

COPY --from=builder /usr/src/codex_gigas/target/release/codex_gigas* ./
EXPOSE 8000
CMD ["./codex_gigas"]