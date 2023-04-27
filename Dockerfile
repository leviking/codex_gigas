FROM rust:1.69-alpine
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/codex_gigas
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD ["./target/release/codex_gigas"]

