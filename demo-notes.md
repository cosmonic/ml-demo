# Demo notes

This was tested on macos. It should work on linux with little or no changes.

This uses 'cosmo up', so you need a cosmonic account.

## Setup

- check out code

    ```
    git clone -b ss-demo https://github.com/stevelr/wasmCloudArtefacts ml-demo
    cd ml-demo
    ```
- make sure you have the latest rust

    ```
    rustup update
    ```

- install cosmo-cli and wash-cli

- install bindle and bindle-server with 

    ```
    cargo install --features=cli,default --bins --git https://github.com/deislabs/bindle
    ```

- check for other dependencies. Fix any errors

    ```
    deploy/checkup.sh
    ```

- edit deploy/env to fit your environment and paths

- compile everything. From the top folder,

    ```
    make
    ```
    (ignore deprecation warnings referring to "a future version of Rust")

## Start services

It's best to start with a clean slate - no wasmcloud hosts running on your local machine

```
cd deploy

# these commands start a local bindle server and load models into it.
# Once they are loaded, you can run a future demo with just the bindle-start command
./run-demo.sh bindle-start
./run-demo.sh load-models

# start your local server and install everything
./run-demo.sh all
  
```

If there are no errors, the last command above should end with a host inventory showing 5 actors and 2 capability providers.
They should also show up on your cosmonic dashboard.

## Demo

open a web browser to http://127.0.0.1:8079/
Select priority:Accuracy, model:Mobilenet v2.7,
then select a picture with the "Browse" button.
Sample images are located in mldemo/images

(ignore the number images 0.png etc. They are for digit recognition demo)

Whenever the image is changed, it is sent through the recognition pipeline.
One quirk of this page is that a new evaluation is only triggered on change of the image. not any of the other fields, so if you want to use the same image with a different model, you need to change the model, then the image to something else, then change it back again.

The resnet model almost always has higher accuracy than mobilenet.

 
    
