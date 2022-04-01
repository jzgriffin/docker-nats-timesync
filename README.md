# NATS Time Sync

This is a single Docker container that demonstrates four things:
  1. Building Rust code for Alpine as a multistage build.
  2. Layering the NATS broker into a Docker image with other programs.
  3. Running a simple NATS pub/sub loop written in Rust.

It creates a Docker container that, when executed, sets the system time
to itself every 500ms.  It does this by publishing the local system time
from the `publisher` program to the `time.system` NATS topic every
500ms.  The `subscriber` program listens for `time.system` messages from
NATS and sets the system time using `settimeofday` to the received time.

## Setup

This package only requires Docker.  The entire build process is done
inside of the Docker build script.  If you do not have Docker installed,
you can use the `setup.sh` script to install it on Ubuntu-based systems
by running this command:

```
sudo ./setup.sh
```

## Running

Once Docker is installed, you can build and run the container with the
`run.sh` script:

```
./run.sh
```

You will see output similar to:

```
Connecting to NATS URL localhost
Connecting to NATS URL localhost
[7] 2022/04/01 21:55:21.992677 [INF] Starting nats-server
[7] 2022/04/01 21:55:21.992698 [INF]   Version:  2.7.4
[7] 2022/04/01 21:55:21.992701 [INF]   Git:      [a86b84a]
[7] 2022/04/01 21:55:21.992711 [INF]   Name:     NDZ4EWP6AVLSNXVAMZTTU2EEZ7JTOWHLJ5I4RTJC55TLPESMKXWSQKMT
[7] 2022/04/01 21:55:21.992716 [INF]   ID:       NDZ4EWP6AVLSNXVAMZTTU2EEZ7JTOWHLJ5I4RTJC55TLPESMKXWSQKMT
[7] 2022/04/01 21:55:21.992734 [INF] Using configuration file: /etc/nats/nats-server.conf
[7] 2022/04/01 21:55:21.993303 [INF] Starting http monitor on 0.0.0.0:8222
[7] 2022/04/01 21:55:21.993367 [INF] Listening for client connections on 0.0.0.0:4222
[7] 2022/04/01 21:55:21.993617 [INF] Server is ready
[7] 2022/04/01 21:55:21.993632 [INF] Cluster name is my_cluster
[7] 2022/04/01 21:55:21.993649 [INF] Listening for route connections on 0.0.0.0:6222
Sending timestamp 1648850122661011
Sending timestamp 1648850123161098
Subscribing to subject 'time.system'
Sending timestamp 1648850123661180
Received timestamp 1648850123661180; local time went from 2022-04-01 21:55:23.661406358 +00:00 to 2022-04-01 21:55:23.661198665 +00:00
Sending timestamp 1648850124160815
Received timestamp 1648850124160815; local time went from 2022-04-01 21:55:24.160996095 +00:00 to 2022-04-01 21:55:24.160833615 +00:00
```

## Known Problems

  - Modifies the host computer's time. This is a limitation of Docker.
  - Does not properly cache the Cargo crate directory.  This causes the
    entire dependency graph to be rebuilt on each Docker build.
