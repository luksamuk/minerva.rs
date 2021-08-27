#!/bin/bash
rm .env
# DATABASE_URL definido via configuração externa
export ROCKET_KEEP_ALIVE=0
cargo install diesel_cli --no-default-features --features postgres && diesel migration run
exec ./target/release/minerva

