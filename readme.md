# Installation

The `VowpalWabbit-sys` crate searches for a library called `vw_rs_bindings`. This needs to be present on your system in order to use the bindings. To install it on Windows/MacOS/Linux the following should. It handles dependencies using vcpkg.

```sh
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cmake --preset=vcpkg
cmake --build --preset=vcpkg
sudo cmake --install build --strip
```

After installing you may need to update the runtime search path to make sure it is findable.

Ubuntu
```sh
export LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib
```

Windows

```powershell
$env:Path += ';C:/Program Files (x86)/vowpalwabbit-rs-bindings/bin/'
```

After this initial step, the `VowpalWabbit-sys` and `VowpalWabbit` crates should function as expected.
