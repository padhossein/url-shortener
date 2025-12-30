From rust:1-alpine AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
FROM alpine:latest
RUN apk --no-cache add sqlite
COPY --from=builder /usr/src/app/target/release/url-shortener .
COPY --chown=root:root urls.db* .
ENV DATABASE_URL=sqlite://urls.db?mode=rwc
EXPOSE 3000
CMD ["./url-shortener"]