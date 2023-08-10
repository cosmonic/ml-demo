#!/usr/bin/env bash

_DIR=$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)
source "$_DIR/../../deploy/env"

# Do not forget to clean bindle's cache:
# rm -rf ~/.cache/bindle

$BINDLE_SERVER --unauthenticated

