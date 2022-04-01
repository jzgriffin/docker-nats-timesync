##
# Builder stage

FROM rust:alpine AS builder

RUN apk add --no-cache libc-dev git gcc make

# Build faketime
# This is necessary to use time-setting system calls, like settimeofday,
# inside of a Docker container.
WORKDIR /usr/src
RUN git clone https://github.com/wolfcw/libfaketime.git && \
    cd libfaketime && \
    git checkout v0.9.10 && \
    make

# Build the pub/sub programs
COPY ./ /usr/src/app/
WORKDIR /usr/src/app
RUN cargo build --release
RUN cargo install --path publisher
RUN cargo install --path subscriber

##
# Deployment stage

FROM nats:alpine

COPY --from=builder /usr/src/libfaketime/src/libfaketime.so.1 /usr/local/lib/
ENV LD_PRELOAD=/usr/local/lib/libfaketime.so.1

COPY --from=builder /usr/local/cargo/bin/publisher /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/subscriber /usr/local/bin/

COPY entrypoint.sh /usr/local/bin/
ENTRYPOINT ["entrypoint.sh"]
