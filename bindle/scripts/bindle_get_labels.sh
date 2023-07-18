#!/usr/bin/env bash

_DIR=$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)
source "$_DIR/../../deploy/env"

if [ $# -eq 0 ]
  then
    echo -n "call this script with at least one argument, for example: "
    echo "$0 ../models/identity_input_output.json ../models/identity_input_output.onnx"
fi

for i in $*; do 
  $BINDLE generate-label $i 
done
