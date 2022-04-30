#!/bin/sh
docker run -p 8000:8000 \
       -e DATABASE_URL=postgres://SYSDBA:masterkey@localhost/minervadb \
       -e REDIS_URL=redis://localhost:6379 \
       --net="host" \
       minerva:latest
