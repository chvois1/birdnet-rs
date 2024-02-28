# TFLite server example

## Prereqreuisites

In order to run this example, you will first install WasmEdge with Tensorflow Lite plugin:

```bash
VERSION=0.13.1
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v $VERSION --plugins wasi_nn-tensorflowlite
```

Then, install Tensorflow Lite dependency libraries:

```bash
VERSION=TF-2.12.0-CC
curl -s -L -O --remote-name-all https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/$VERSION/WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
tar -zxf WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
rm -f WasmEdge-tensorflow-deps-TFLite-$VERSION-manylinux2014_x86_64.tar.gz
mv libtensorflowlite_c.so ~/.wasmedge/lib
mv libtensorflowlite_flex.so ~/.wasmedge/lib


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

## Emscripten

[Emscripten](https://emscripten.org/docs/compiling/Building-Projects.html)
 is the compiler toolchain to WebAssembly. It can compile C and C++ code, or any other language that uses LLVM, into WebAssembly, and run it on the Web, Node.js, or other Wasm runtimes.

```bash
./emsdk activate latest
source "/home/s227840/Projets/BirdNET/emsdk/emsdk_env.sh"
```

## Build tensorflow lite

```bash
[tensorflow](https://www.tensorflow.org/lite/guide/build_cmake?hl=fr)
apt-get install clang-9
apt-get install clang-format
git clone https://github.com/tensorflow/tensorflow.git tensorflow_src
mkdir build_tensorflow
cd build_tensorflow
emcmake cmake -DCMAKE_ASM_COMPILER="clang-9" -DCMAKE_ASM_COMPILER_ID="Clang" -DTFLITE_ENABLE_XNNPACK=OFF -DCMAKE_BUILD_TYPE=Debug ../tensorflow_src/tensorflow/lite
../tensorflow_src/tensorflow/lite
emmake make VERBOSE=1
```

## Build minimal example

[minimal](https://github.com/tensorflow/tensorflow/tree/master/tensorflow/lite/examples/minimal)

```bash
mkdir build_minimal
cd build_minimal
emcmake cmake -DCMAKE_ASM_COMPILER="clang-9" -DCMAKE_ASM_COMPILER_ID="Clang" -DTFLITE_ENABLE_XNNPACK=OFF -DCMAKE_BUILD_TYPE=Debug ../tensorflow_src/tensorflow/lite/examples/minimal
emmake make VERBOSE=1
```
