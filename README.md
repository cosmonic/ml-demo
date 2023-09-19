# MlInference

> **_NOTE:_** additional documentation [here](https://finfalter.github.io/wasmCloudArtefacts/)

This repository provides a [wasmCloud](https://wasmcloud.dev/)
capability provider and actors to perform **inference**
using machine learning models for ONNX and Tensorflow.

## Usage

In order to run and deploy the ML Demo on Cosmonic, follow the [Cosmonic Getting Started Guide](https://cosmonic.com/docs/user_guide/cli/getting_started#installing-the-cli) and use the `cosmo` CLI to deploy the application.

```bash
cosmo up -d

# Using wadm manifest: https://github.com/wasmCloud/wadm
cosmo app put wadm.yaml
cosmo app deploy ml-app
```



## Examples

Apart from the underlying inference engine, e.g. ONNX vs. Tensorflow, the pre-configured models differ in a further aspect: concerning the _trivial_ models, one may request processing upon arbitrary shapes of one-dimensional data, `[1, n]`. [Mobilenet](https://github.com/onnx/models/tree/main/vision/classification/mobilenet) and [Squeezenet](https://github.com/onnx/models/tree/main/vision/classification/squeezenet), however, have more requirements regarding their respective input tensor. To fulfill these, the respective input tensor of an arbitrary image can be preprocessed before being routed to the inference engine.

The application provides three endpoints. The first endpoint routes the input tensor to the related inference engine without any pre-processing. The second endpoint **pre-processes** the input tensor and routes it to the related inference engine thereafter. The third performs a pre-processing before the prediction step and a **post-processinging** afterwards.

1. `0.0.0.0:<port>/<model>`, e.g. `0.0.0.0:7078/identity`
2. `0.0.0.0:<port>/<model>/preprocess`, e.g. `0.0.0.0:7078/squeezenetv117/preprocess`
3. `0.0.0.0:<port>/<model>/matches`, e.g. `0.0.0.0:7078/squeezenetv117/matches`

### Identity Model

To trigger a request against the **_identity_** model, type the following:

```bash
curl -v POST 0.0.0.0:8078/identity -d '{"dimensions":[1,4],"valueTypes":["ValueF32"],"flags":0,"data":[0,0,128,63,0,0,0,64,0,0,64,64,0,0,128,64]}'
```

The response should comprise `HTTP/1.1 200 OK` as well as `{"result":"Success","tensor":{"dimensions":[1,4],"valueTypes":["ValueF32"],"flags":0,"data":[0,0,128,63,0,0,0,64,0,0,64,64,0,0,128,64]}}`

The following happens:

1. The http **_POST_** sends a request for a model with name _"challenger"_, index `0` and some `data`.
2. `data` is vector `[1.0f32, 2.0, 3.0, 4.0]` converted to a vector of bytes.
3. A response is computed. The result is sent back.
4. The `data` in the request equals `data` in the response because the pre-loaded model "_challenger_" is one that yields as output what it got as input.

### Plus3 model

To trigger a request against the **_plus3_** model, type the following:

```bash
curl -v POST 0.0.0.0:8078/plus3 -d '{"dimensions":[1,4],"valueTypes":["ValueF32"],"flags":0,"data":[0,0,128,63,0,0,0,64,0,0,64,64,0,0,128,64]}'
```

The response is

```bash
{"result":"Success","tensor":{"dimensions":[1,4],"valueTypes":["ValueF32"],"flags":0,"data":[0,0,128,64,0,0,160,64,0,0,192,64,0,0,224,64]}}
```

Note that in contrast to the **_identity_** model, the answer from **_plus3_** is not at all identical to the request. Converting the vector of bytes `[0,0,128,64,0,0,160,64,0,0,192,64,0,0,224,64]` back to a vector of `f32` yields `[4.0, 5.0, 6.0, 7.0]`. This was expected: each element from the input is incremented by three `[1.0, 2.0, 3.0, 4.0]` &rarr; `[4.0, 5.0, 6.0, 7.0]`, hence the name of the model: **_plus3_**.

### Mobilenet model

```bash
# in order for the relative path to match call from directory 'deploy'
curl -v POST 0.0.0.0:8078/mobilenetv27/preprocess --data-binary @../providers/mlinference/tests/testdata/images/n04350905.jpg
```

Note that the output tensor is of shape `[1,1000]` and needs to be post-processed by an evaluation of the [softmax](https://en.wikipedia.org/wiki/Softmax_function) over the outputs. In case the softmax shall be evaluated as well use the third endpoint, for example like the following:

```bash
# in order for the relative path to match call from directory 'deploy'
curl -v POST 0.0.0.0:8078/mobilenetv27/matches --data-binary @../providers/mlinference/tests/testdata/images/n04350905.jpg
```

### Squeezenet model

```bash
# in order for the relative path to match call from directory 'deploy'
curl -v POST 0.0.0.0:8078/squeezenetv117/preprocess --data-binary @../providers/mlinference/tests/testdata/images/n04350905.jpg
```

Note that the output tensor is of shape `[1,1000]` and needs to be post-processed where the post-processing is currently not part of the application. Or, including pos-processing as follows:

```bash
# in order for the relative path to match call from directory 'deploy'
curl -v POST 0.0.0.0:8078/squeezenetv117/matches --data-binary @../providers/mlinference/tests/testdata/images/n04350905.jpg
```

The answer should comprise

```bash
[{"label":"n02883205 bow tie, bow-tie, bowtie","probability":0.16806115},{"label":"n04350905 suit, suit of clothes","probability":0.14194612},{"label":"n03763968 military uniform","probability":0.11412828},{"label":"n02669723 academic gown, academic robe, judge's robe","probability":0.09906072},{"label":"n03787032 mortarboard","probability":0.09620707}]
```

## Creation of new bindles

The capability provider assumes a bindle to comprise two parcels where each parcel is assigned one of the following two groups:

- **_model_**
- **_metadata_**

The first, `model`, is assumed to comprise model data, e.g. an ONNX model. The second, `metadata`, is currently assumed to be json containing the metadata of the model. In case you create new bindles, make sure to assign these two groups.

## Supported Inference Engines

The capability provider uses the amazing inference toolkit [tract](https://github.com/sonos/tract) and currently supports the following inference engines

1. [ONNX](https://onnx.ai/)
2. [Tensorflow](https://www.tensorflow.org/)

### Restrictions

Concerning ONNX, see [tract's documentation](https://github.com/sonos/tract) for a detailed discussion of ONNX format coverage.

Concerning Tensorflow, only TensorFlow 1.x is supported, not Tensorflow 2. However, models of format Tensorflow 2 may be converted to Tensorflow 1.x. For a more detailled discussion, see the following resources:

- `https://www.tensorflow.org/guide/migrate/tf1_vs_tf2`
- `https://stackoverflow.com/questions/59112527/primer-on-tensorflow-and-keras-the-past-tf1-the-present-tf2#:~:text=In%20terms%20of%20the%20behavior,full%20list%20of%20data%20types.`

Currently, there is no support of any accelerators like GPUs or TPUs. On the one hand, there is a range of [coral devices](https://coral.ai/products/) like the [Dev board](https://coral.ai/docs/dev-board/get-started) supporting Tensorflow for TPU based inference. However, they only support the [Tensorflow Lite](https://www.tensorflow.org/lite) derivative. For more information see Coral's [Edge TPU inferencing overview](https://coral.ai/docs/edgetpu/inference/).
