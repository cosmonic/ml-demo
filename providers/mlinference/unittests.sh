#!/usr/bin/env bash

ps -ef | grep mlinference | grep -v grep | awk '{print $2}' | xargs kill
ps -ef | grep wasmcloud   | grep -v grep | awk '{print $2}' | xargs kill

export RUST_LOG=debug,warp=info

cargo test