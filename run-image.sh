#!/bin/sh
docker run \
       -e DATABASE_URL=postgres://SYSDBA:masterkey@localhost/minervadb \
       -e REDIS_URL=redis://localhost:6379 \
       --net="host" \
       minerva:latest
