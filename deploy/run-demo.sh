#!/usr/bin/env bash
set -e

_DIR=$(cd $(dirname "${BASH_SOURCE[0]}") && pwd)

show_help() {
cat <<_SHOW_HELP
  This program runs the ml inference demo. Useful commands:

   $0 load-models                  - start bindle server and load models (run this _before_ 'all')
   $0 all                          - run everything
   $0 wipe                         - stop everything and erase all secrets

   $0 inventory                    - show host inventory

Custom environment variables and paths should be set in ${_DIR}/env
_SHOW_HELP
}

# multitail -iw "~/opt/wasmcloud/tmp/log/erlang.log.*" 1

## ---------------------------------------------------------------
## START CONFIGURATION
## ---------------------------------------------------------------

WASH=${WASH:-wash}
WASMCLOUD_HOST=${WASMCLOUD_HOST:-wasmcloud}
WASMCLOUD_JS_DOMAIN=${WASMCLOUD_JS_DOMAIN:-cosmonic}

check=$(printf '\342\234\224\n' | iconv -f UTF-8)

if [ ! -f "$_DIR/env" ]; then
    echo "Missing $_DIR/env file"
    exit 1
fi
source "$_DIR/env"


# allow extra time to process RPC
export WASMCLOUD_RPC_TIMEOUT_MS=8000
# enable verbose logging
export WASMCLOUD_STRUCTURED_LOGGING_ENABLED=true
export WASMCLOUD_STRUCTURED_LOG_LEVEL=debug
#export RUST_LOG=${RUST_LOG:-RUST_LOG=debug,hyper=info,oci_distribution=info,reqwest=info}

##
#   BINDLE
## 

BINDLE_START_SCRIPT="${_DIR}/../bindle/scripts/bindle_start.sh"
BINDLE_CREATION_SCRIPT="${_DIR}/../bindle/scripts/bindle_create.sh"
BINDLE_SHUTDOWN_SCRIPT="${_DIR}/../bindle/scripts/bindle_stop.sh"

##
#   WASMCLOUD HOST
# (defined in env)
##

##
#   CAPABILITY PROVIDERS
##

# oci registry - as used by wash
REG_SERVER=${HOST_DEVICE_IP}:5000

# registry server as seen by wasmcloud host. use "registry:5000" if host is in docker
REG_SERVER_FROM_HOST=${HOST_DEVICE_IP}:5000

HTTPSERVER_REF=wasmcloud.azurecr.io/httpserver:0.18.2
HTTPSERVER_ID=VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
#HTTPSERVER_REF=${HOST_DEVICE_IP}:5000/v2/httpserver:0.18.2
#HTTPSERVER_ID=VDWKHKPIIORJM4HBFHL2M7KZQD6KMSQ4TLJOCS6BIQTIT6S7E6TXGLIP

# actor to link to httpsrever. 
INFERENCEAPI_ACTOR=${_DIR}/../actors/inferenceapi

IMAGE_UI_ACTOR=${_DIR}/../actors/image-ui

# http configuration file. use https_config.json to enable TLS
INFERENCE_ADDR=0.0.0.0:8078
IMAGE_UI_ADDR=0.0.0.0:8079

MODEL_CONFIG=actor_config.json

# command to base64 encode stdin to stdout
BASE64_ENC=base64

# where passwords are stored after being generated
SECRETS=.secrets
#PSQL_ROOT=.psql_root
#PSQL_APP=.psql_app
#CREATE_APP_SQL=.create_app.sql
CLUSTER_SEED=.cluster.nk

#ALL_SECRET_FILES="$SECRETS $PSQL_ROOT $PSQL_APP $SQL_CONFIG $CREATE_APP_SQL $CLUSTER_SEED"
ALL_SECRET_FILES="$SECRETS $CLUSTER_SEED"

## ---------------------------------------------------------------
## END CONFIGURATION
## ---------------------------------------------------------------

start_host () {
    echo "starting nats server and wasmCloud host"
    WASMCLOUD_OCI_ALLOWED_INSECURE=${REG_SERVER_FROM_HOST} cosmo up
}

# stop the local host and all actors/providers
stop_host() {

    cosmo down

    ps -ef | grep mlinference | grep -v grep | awk '{print $2}' | xargs -r kill
    ps -ef | grep wasmcloud   | grep -v grep | awk '{print $2}' | xargs -r kill
    killall -q -KILL wasmcloud_httpserver_default || true
    killall -q -KILL wasmcloud_mlinference_default || true
}


# stop docker and wipe all data (database, nats cache, host provider/actors, etc.)
wipe_all() {

    cat >$SECRETS <<__WIPE
WASMCLOUD_CLUSTER_SEED=
WASMCLOUD_CLUSTER_SEED=
__WIPE

    stop_host
    docker-compose --env-file $SECRETS rm -sf registry

    rm -f $ALL_SECRET_FILES

    ${WASH} drain all

    # clear bindle cache
    rm -rf ~/.cache/bindle ~/Library/Caches/bindle
}

create_seed() {
    local _seed_type=$1
    ${WASH} keys gen -o json $_seed_type | jq -r '.seed'
}

create_secrets() {
    #root_pass=$($MKPASS)
    #app_pass=$($MKPASS)

    cluster_seed=$(create_seed Cluster)
    echo $cluster_seed >$CLUSTER_SEED

cat > $SECRETS << __SECRETS
WASMCLOUD_CLUSTER_SEED=$cluster_seed
__SECRETS

    # protect secret files
    chmod 600 $ALL_SECRET_FILES
}

start_bindle() {
    printf "\n[bindle-server startup]\n"
    ML_BINDLE_ADDR=$ML_BINDLE_ADDR \
        BINDLE_DIRECTORY=$ML_BINDLE_DIR \
        BINDLE_KEYRING=$ML_BINDLE_KEYRING \
        $BINDLE_START_SCRIPT
}

#stop_bindle() {
#    printf "\n[bindle-server shutdown]\n"
#    bash "$BINDLE_SHUTDOWN_SCRIPT"
#}

create_bindle() {   
    printf "\n[bindle creation]\n"
    BINDLE_URL=$ML_BINDLE_URL \
        BINDLE_DIRECTORY=$ML_BINDLE_DIR \
        BINDLE_KEYRING=$ML_BINDLE_KEYRING \
        $BINDLE_CREATION_SCRIPT
}

# get the host id (requires wasmcloud to be running)
host_id() {
    ${WASH} get hosts -o json | jq -r ".hosts[0].id"
}

# build and push capability provider
push_providers() {
    echo "\npushing capability provider mlinference to local registry .."
    make -C ${_DIR}/../providers/mlinference all push
}

# start docker services
# idempotent
start_services() {

    # ensure we have secrets
    if [ ! -f $SECRETS ]; then
        create_secrets
    fi

    echo "starting containers with nats and registry .."

    docker-compose --env-file $SECRETS up -d registry
    # give things time to start
    
    sleep 4
}

# help preparing remote device
# idempotent
prepare_remote_device() {

    printf "\nTARGET_DEVICE_IP is detected to be remote --> you try to deploy the runtime on a remote node\n\n"
    printf "In order to prepare well you certainly\n"
    printf "$check loaded ${_DIR}/../iot/configure_edge.sh to the remote node\n"
    printf "$check loaded ${_DIR}/../iot/restart_edge.sh to the remote node\n"
    printf "$check 'source ./configure_edge.sh' on the remote node\n"
    printf "$check started NATS ('nats-server --jetstream') on the remote node\n"
    printf "$check started wasmCloud runtime ('restart_edge.sh') on the remote node\n"
    printf "$check 'set HOST_DEVICE_IP in env.sh\n"
    printf "$check 'set TARGET_DEVICE_IP in env.sh\n\n"

    read  -n 1 -p "press any button to start deployment"
}

start_actors() {

    echo "starting actors .."
    _here=$PWD
    cd ${_DIR}/../actors
    for i in */; do
        if [ -f $i/wasmcloud.toml ]; then
            (cd $i && wash build && cosmo launch)
            #make HOST_DEVICE_IP=${HOST_DEVICE_IP} -C $i build push start
        fi
    done
    cd $_here
}

# start wasmcloud capability providers
# idempotent
start_providers() {

    CONFIG_JSON=$(mktemp)
cat >$CONFIG_JSON.tmp <<__JSON
{
 "bindle_url": "$ML_BINDLE_URL",
 "bindle_keyring": "$ML_BINDLE_KEYRING"
}
__JSON
   tr -d '\r\n' <$CONFIG_JSON.tmp >$CONFIG_JSON

    VERSION=$(cd ../providers/mlinference && cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
    echo "starting capability provider mlinference:$VERSION from registry with config $CONFIG_JSON"

    STARGATE_HOST=$(wash get hosts -o json | jq -r '.hosts | select(.[] | .labels | contains({"stargate": "true"}))[0] | .id')
    if [ -z "$STARGATE_HOST" ]; then
       echo "could not find stargate host - can't start provider"
       exit 1
    fi

  	$WASH start provider \
       --host-id $STARGATE_HOST \
       --config-json "$CONFIG_JSON" \
       "$REG_SERVER/v2/mlinference:$VERSION"
    
    echo "starting http server capability provider"
    ${WASH} start provider \
        --host-id $STARGATE_HOST \
        $HTTPSERVER_REF
}

# base-64 encode file into a string
b64_encode_file() {
    cat "$1" | $BASE64_ENC | tr -d ' \r\n'
}

# link actors with providers
# idempotent
link_providers() {
    local inference_api_id
    local image_ui_id
    local _a

    # link inferenceapi actor to http server, so it can be curl'd directly
    inference_api_id=$(make -C $INFERENCEAPI_ACTOR --silent actor_id)
    ${WASH} link put --timeout-ms 4000 $inference_api_id $HTTPSERVER_ID wasmcloud:httpserver address=$INFERENCE_ADDR 

    # use locally-generated id, since mlinference provider isn't published yet
    MLINFERENCE_ID=$(${WASH} inspect -o json ${_DIR}/../providers/mlinference/build/mlinference.par.gz | jq -r '.service')

    # link inferenceapi actor to mlinference provider
    ${WASH} link put --timeout-ms 4000 $inference_api_id $MLINFERENCE_ID wasmcloud:mlinference "config_b64=$(b64_encode_file $MODEL_CONFIG)"

    # link image-ui actor to http server on its own port
    image_ui_id=$(make -C $IMAGE_UI_ACTOR --silent actor_id)
    ${WASH} link put --timeout-ms 4000 $image_ui_id $HTTPSERVER_ID wasmcloud:httpserver address=$IMAGE_UI_ADDR 
}

show_inventory() {    
    for h in $(wash get hosts -o json | jq -r '.hosts[] | .id'); do
         ${WASH} get inventory $h
    done
}


stop_registry() {
    set +e
    docker ps | grep registry:2 && docker compose stop registry
    docker compose rm -f registry
    set -e
}

start_registry() {
    docker compose up -d registry
}

run_all() {

    # turn off local registry
    stop_registry

    # stop services if they were leftover
    killall -q -KILL wasmcloud_httpserver_default || true
    killall -q -KILL wasmcloud_mlinference_default || true


    # make sure we have all prerequisites installed
    ${_DIR}/checkup.sh

    if [ ! -f $SECRETS ]; then
        create_secrets
    fi

    # start all the containers in case the target is localhost
    if [ "$TARGET_DEVICE_IP" != "127.0.0.1" ]; then
        # help preparing to ramp up the remote device
        prepare_remote_device

        # in case you do not run a local registry, switch it on
        start_registry
    else 

        echo "starting runtime, nats and registry on host"
        start_services

        # start host
        start_host 
        sleep 10 # make sure host is started before next step
    fi

    # build inference capability provider and push to local registry
    push_providers

    # build, push, and start all actors
    start_actors
    sleep 2

    # start capability providers: inference and httpserver 
    start_providers
    sleep 2

    # link providers with actors
    link_providers

    show_inventory
}

case $1 in 

    secrets ) create_secrets ;;
    wipe ) wipe_all ;;
    start ) start_services ;;
    inventory ) show_inventory ;;
    bindle-start | start-bindle ) start_bindle ;;
    #bindle-stop | stop-bindle ) stop_bindle ;;
    load-models | bindle-create | create-bindle ) create_bindle ;;
    start-actors ) start_actors ;;
    push-providers ) push_providers ;;
    start-providers ) start_providers ;;
    link-providers | link-actors ) link_providers ;;
    start-host | host-start ) shift; start_host "$@" ;;
    run-all | all ) shift; run_all "$@" ;;

    * ) show_help && exit 1 ;;

esac

