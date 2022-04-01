#!/usr/bin/env bash

docker build -t docker-nats-timesync . && \
    docker run -t docker-nats-timesync
