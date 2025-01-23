FROM node:alpine as web-builder
RUN apk add pnpm
WORKDIR /usr/src
COPY client/package.json .
COPY client/pnpm-lock.yaml .
RUN pnpm install
COPY client/ .
RUN pnpm run build
RUN pnpm prune --production

FROM rust:alpine as builder
WORKDIR /usr/src/api-service
RUN apk add --no-cache musl-dev

# Update this to whatever database provider you use
RUN apk add --no-cache sqlite-dev sqlite
ENV RUSTFLAGS="-C target-feature=-crt-static" 
RUN cargo install diesel_cli --no-default-features --features sqlite

COPY service/Cargo.lock Cargo.lock
COPY service/Cargo.toml Cargo.toml

COPY service/migrations/ migrations
ENV DATABASE_URL=/usr/src/api-service/db.sqlite3
RUN diesel database setup

COPY service/src/ src
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache sqlite-dev sqlite libgcc ca-certificates 
COPY --from=builder /usr/src/api-service/target/release/service /app
COPY --from=builder /usr/src/api-service/db.sqlite3 /app/db.sqlite3

COPY --from=web-builder /usr/src/build /app/static

ENV STATIC_FILE_PATH=/app/static PORT=8080
# Replace with your database connection string if not using sqlite
ENV DATABASE_URL=/app/db.sqlite3
EXPOSE 8080
CMD ["/app/service"]