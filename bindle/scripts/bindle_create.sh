#!/usr/bin/env bash

BINDLE_SCRIPTS=$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)
#source "${BINDLE_SCRIPTS}/../../deploy/env"

for model in $(cat $BINDLE_SCRIPTS/../models/models.txt); do
  $BINDLE_SCRIPTS/push_bindle.sh $BINDLE_SCRIPTS/../models/$model-signed.toml $BINDLE_SCRIPTS/../models/$model.csv
done

#if [ -f $BINDLE_SCRIPTS/../models/resnet152-v2-7.onnx ]; then
#  source $BINDLE_SCRIPTS/push_bindle.sh $BINDLE_SCRIPTS/../models/resnet152-v2-7-signed.toml $BINDLE_SCRIPTS/../models/resnet152-v2-7.csv
#fi

