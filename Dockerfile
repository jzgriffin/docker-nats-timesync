##
# Builder stage

FROM rust:alpine AS builder

RUN apk add --no-cache libc-dev

# Build the pub/sub programs
COPY ./ /usr/src/app/
WORKDIR /usr/src/app
RUN cargo build --release
RUN cargo install --path publisher
RUN cargo install --path subscriber

##
# Deployment stage

FROM nats:alpine

COPY --from=builder /usr/local/cargo/bin/publisher /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/subscriber /usr/local/bin/

COPY entrypoint.sh /usr/local/bin/
ENTRYPOINT ["entrypoint.sh"]
