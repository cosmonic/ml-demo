# Contributing

## Setup

- check out code

```bash
git clone -b ss-demo https://github.com/stevelr/wasmCloudArtefacts ml-demo
cd ml-demo
```

- make sure you have the latest rust

```bash
rustup update
```

- install cosmo-cli and wash-cli

## Build actors and providers

From the top-level **directory** build with `make`. This should complete
without errors.

## Running locally-built components

It's best to start with a clean slate - no wasmcloud hosts running on your local machine

The script `deploy/run-demo.sh` contains commands to run everything. In the
`deploy` folder, run `run-demo.sh` to see a list of available subcommands.

Start the local registry server, nats server, wasmcloud host,
actors, and providers. If this is your first time running running this
app, add `--console` to the end of the following command to open a new
terminal window with the host logs. The logs may be useful for
diagnosing any problems.

```bash
./run-demo.sh all
# or, to open a $TERMINAL window with host logs
./run-demo.sh all --console
```

After a successful startup, the inventory should look similar to the following:
```bash
wash get inventory

                                                                                         
                                          Host Inventory (NAZWBNEAMWM6B5EHTCRXKNI3LKYL3HFINT44CW7QRL3KVRH2UEJA4ALW)
                                                                                         
  hostcore.os                                                                            windows
  hostcore.osfamily                                                                      windows
  hostcore.arch                                                                          x86_64
  stargate                                                                               true
                                                                                         
  Actor ID                                                    Name                       Image Reference
  MDQTNL4LURMI5JKLFJNIVJQWQMGDJTXE7MWD7ASXP7PRFYAJQJBIYNG3    ML Inference UI            ghcr.io/liamrandall/image-ui:0.2.0
  MBZDEWD27BW6REESEOJ2EWT4GWVFCYQCYVS7D23Y53R5TUB65DIPV5UQ    ml_imagenetpreprocessor    ghcr.io/liamrandall/imagenetpreprocessor:0.1.0
  MAHR7434ZSCMRNY2T3W6AP3X2WZUJFSPHOF6HCB2EHSP3TTTZOVQJBZ3    ml_imagenetpostprocessor   ghcr.io/liamrandall/imagenetpostprocessor:0.1.0
  MBAS33Z3NX7K3OYDOIXORQHLUSAYMHDJB37B3HARJ27IJUWWQL46KUKR    ml_imagenetpreprocrgb8     ghcr.io/liamrandall/imagenetpreprocrgb8:0.1.0
  MA7ZIVAJMZSRVVKMFQQNSH4WQWKGE25MEUY7XRMGEHD3HCK5OAKWNJZH    inferenceapi               ghcr.io/connorsmith256/inferenceapi:0.2.0
                                                                                         
  Provider ID                                                 Name                       Link Name                Image Reference
  VBPGEQMDSZO35ERCCYMLMR3RQK7K5MMJPCSQNYOK37W4MQAEWUYDRSVZ    mlinference                default                  ghcr.io/connorsmith256/mlinference:0.2.1
```

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

## Demo

1. Select "priority:Accuracy", "model:Mobilenet v2.7"
1. Then select a picture with the "Browse" button.
1. Sample images are located in ml-demo/images
1. Whenever the image is changed, it is sent through the recognition pipeline.
1. One quirk of this page is that a new evaluation is only triggered on change of the image. Not any of the other fields, so if you want to use the same image with a different model, you need to change the model, then the image to something else, then change it back again.
1. The resnet model almost always has higher accuracy than mobilenet.
