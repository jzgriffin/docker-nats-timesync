#!/bin/sh

nats-server --config /etc/nats/nats-server.conf &
subscriber &
publisher &

# Wait for any of the above processes to exit, then relay the exit code
wait -n
exit $?
