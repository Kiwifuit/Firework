FROM rust:1.81.0-alpine AS build

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev

# TODO: Add dms feature when stable :>
RUN cargo build -rp dms-backend

FROM alpine:latest
COPY --from=build /app/target/release/dms-backend /usr/bin/

CMD ["dms-backend"]