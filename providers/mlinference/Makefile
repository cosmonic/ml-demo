# mlinference Makefile

CAPABILITY_ID = "wasmcloud:mlinference"
NAME = mlinference
VENDOR = "wasmCloud"
PROJECT = mlinference
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)

REVISION = 0
oci_url_base = 127.0.0.1:5000/v2
oci_insecure = "--insecure"

include ./provider.mk

test::
	cargo clippy --all-targets --all-features


#STARGATE_HOST=$(shell wash get hosts -o json | jq -r '.hosts | map(select(.labels.stargate == "true"))[0] | .id')


# 'make start' starts on the first host found
# 'make start-local' starts the provider on the stargate host
start-local:
	$(WASH) start provider --constraint stargate=true "$(oci_url_base)/$(NAME):$(VERSION)"
