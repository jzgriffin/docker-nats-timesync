# NATS Time Sync

This is a single Docker container that demonstrates four things:
  1. Building Rust code for Alpine as a multistage build.
  2. Layering the NATS broker into a Docker image with other programs.
  3. Running a simple NATS pub/sub loop written in Rust.
  4. Using faketime to enable `settimeofday` within a Docker image.

It creates a Docker container that, when executed, sets the system time
to itself every 500ms.  It does this by publishing the local system time
from the `publisher` program to the `time.system` NATS topic every
500ms.  The `subscriber` program listens for `time.system` messages from
NATS and sets the system time using `settimeofday` to the received time.
Note that, because Docker blocks access to the system time by default,
the `settimeofday` call is actually implemented by faketime.

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
