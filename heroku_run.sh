#!/bin/bash
rm .env
# DATABASE_URL definido via configuração externa
export ROCKET_PORT=$PORT
export ROCKET_KEEP_ALIVE=0
exec ./target/release/minerva

