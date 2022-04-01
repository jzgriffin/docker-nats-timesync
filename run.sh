#!/usr/bin/env bash

docker build -t docker-nats-timesync . && \
    docker run --cap-add=SYS_TIME -t docker-nats-timesync
