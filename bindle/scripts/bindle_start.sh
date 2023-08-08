#!/usr/bin/env bash

if [ -z "$ML_BINDLE_ADDR" ] || [ -z "$BINDLE_KEYRING" ] || [ -z "$BINDLE_DIRECTORY" ]; then
  echo Missing bindle configuration. Required: ML_BINDLE_ADDR, BINDLE_KEYRING, BINDLE_DIRECTORY
  exit 1
fi

bindle-server --unauthenticated -e \
    --address $ML_BINDLE_ADDR \
    --keyring "$BINDLE_KEYRING" \
    --directory "$BINDLE_DIRECTORY" \
    --strategy CreativeIntegrity &

sleep 2

$BINDLE \
    --server "http://$ML_BINDLE_ADDR/v1/" \
    --keyring "$BINDLE_KEYRING" \
    keys fetch
