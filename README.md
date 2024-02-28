# TFLite server example

## Prereqreuisites

In order to run this example, first install WasmEdge with its Tensorflow Lite plugin:

```bash
VERSION=0.13.1
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v $VERSION --plugins wasi_nn-tensorflowlite
```

Then, install Tensorflow Lite dependency libraries with one of the following two commands depending on the platform type.

For an X86_64 linux platform type:

```bash
VERSION=TF-2.12.0-CC
curl -s -L -O --remote-name-all https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/$VERSION/WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
tar -zxf WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
rm -f WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
mv libtensorflowlite_c.so ~/.wasmedge/lib
mv libtensorflowlite_flex.so ~/.wasmedge/lib

For an ARM_64 linux platform type:

curl -s -L -O --remote-name-all https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/$VERSION/WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_aarch64.tar.gz
tar -zxvf WasmEdge-tensorflow-deps-TFLite-TF-2.12.0-CC-manylinux2014_aarch64.tar.gz
rm -f WasmEdge-tensorflow-deps-TFLite-TF-2.12.0-CC-manylinux2014_aarch64.tar.gz
mv libtensorflowlite_c.so ~/.wasmedge/lib
mv libtensorflowlite_flex.so ~/.wasmedge/lib
```

## Build

```bash
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```

## Run

```bash
wasmedge target/wasm32-wasi/release/birdnet.wasm
```

## Test

Run the following from another terminal.

```bash
curl http://localhost:8081/classify -X POST --data-binary '@soundscape.wav'
```
