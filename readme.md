# Installation

The `VowpalWabbit-sys` crate searches for a library called `vw_rs_bindings`. This needs to be present on your system in order to use the bindings. To install it on Windows/MacOS/Linux the following should. It handles dependencies using vcpkg.

```sh
git submodule update binding/external/vowpal_wabbit
cd binding/external/vowpal_wabbit
git submodule update ext_libs/vcpkg
cd ../../..

cmake --preset=vcpkg
cmake --build --preset=vcpkg
sudo cmake --install build
```

After this initial step, the `VowpalWabbit-sys` and `VowpalWabbit` crates should function as expected.