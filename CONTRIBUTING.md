# Contributing

## Prerequisites

## Build actors and providers

From the top-level **directory** build with `make`. This should complete
without errors.

### Prepare models

Models (in `bindle/models`) must be loaded into the bindle server.

If you are using your own model, you will need to create a "bindle
invoice", a `.toml` file listing the bindle artifacts. Each artifact
has a sha256 hash and file size of each artifact. See the
existing toml files in `bindle/models` for examples.

## Configuration

Update paths in file `deploy/env` to match your development environment.

## Running

The script `deploy/run.sh` contains commands to run everything. In the
`deploy` folder, run `run.sh` to see a list of available subcommands.

Start the local registry server, nats server, wasmcloud host,
actors, and providers. If this is your first time running running this
app, add `--console` to the end of the following command to open a new
terminal window with the host logs. The logs may be useful for
diagnosing any problems.

```bash
./run.sh all
# or, to open a $TERMINAL window with host logs
./run.sh all --console
```

After a successful startup the _washboard_ should look similar to the following screenshot:

<div style="width: 80%; height: 50%">
![washboard after successful launch](images/washboard.png "washboard after successful launch")
</div>

If everything started correctly, try sending an image to be classified:
(try any of the images in `images/`, or try one of your own!

```bash
curl -T images/cat.jpg http://localhost:8078/mobilenetv27/matches | jq
```

To stop the host and providers,

```bash
/run.sh wipe
```

Once the application is up and running, start to issue requests. Currently, the repository comprises the following pre-configured models:

- **_identity_** of ONNX format
- **_plus3_** of Tensorflow format
- **_mobilenet_** of ONNX format
- **_squeezenet_** of ONNX format
