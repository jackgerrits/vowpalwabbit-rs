# Setup

The `VowpalWabbit-sys` crate searches for a library called `vw_rs_bindings`. This needs to be findable on your system in order to use the bindings.

## Build

### Ubuntu

```sh
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cd binding
cmake --preset=vcpkg
cmake --build --preset=vcpkg
```

### MacOS

```sh
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cd binding
cmake --preset=vcpkg
cmake --build --preset=vcpkg
```

### Windows

The following MUST be run in an x64 developer tools prompt. Not using 64 bit will cause issues later when the DLL is loaded.

```powershell
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cd binding
cmake --preset=vcpkg-windows
cmake --build --preset=vcpkg-windows
```

## Make findable in build dir

Set environment variable `VW_RS_BINDING_HOME` to the `binding/build` directory.

On Windows add the `binding/build` directory to the `path`.
On MacOS or Ubuntu set `LD_LIBRARY_PATH` to `binding/build`.

## Install (optional)

### Ubuntu
```sh
sudo cmake --install build --strip

# Make sure this is set when running cargo
export LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib
```

### MacOS
```sh
sudo cmake --install build --strip
```

### Windows

The following MUST be run in an admin prompt.
```powershell
cmake --install build --strip
```

In powershell:
```powershell
# Assumes C drive
$env:Path += ';C:/Program Files (x86)/vowpalwabbit-rs-bindings/bin/'
[System.Environment]::SetEnvironmentVariable('VW_RS_BINDING_HOME','C:/Program Files (x86)/vowpalwabbit-rs-bindings',[System.EnvironmentVariableTarget]::User)
```
