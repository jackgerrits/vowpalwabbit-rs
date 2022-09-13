# Setup

The `VowpalWabbit-sys` crate searches for a library called `vw_rs_bindings`. This needs to be present on your system in order to use the bindings. To install it on Windows/MacOS/Linux the following should. It handles dependencies using vcpkg.

## Ubuntu
```sh
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cd binding
cmake --preset=vcpkg
cmake --build --preset=vcpkg
sudo cmake --install build --strip

# Make sure this is set when running cargo
export LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib
```

## MacOS
```sh
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cmake --preset=vcpkg
cmake --build --preset=vcpkg
sudo cmake --install build --strip
```

## Windows

The following MUST be run in an admin x64 developer tools prompt. Not using 64 bit will cause issues later when the DLL is loaded.
```powershell
git submodule update --init binding/external/vowpal_wabbit
git submodule update --init binding/external/vcpkg
cmake --preset=vcpkg-windows
cmake --build --preset=vcpkg-windows
cmake --install build --strip
```

In powershell:
```powershell
# Assumes C drive
$env:Path += ';C:/Program Files (x86)/vowpalwabbit-rs-bindings/bin/'
[System.Environment]::SetEnvironmentVariable('VW_RS_BINDING_HOME','C:/Program Files (x86)/vowpalwabbit-rs-bindings',[System.EnvironmentVariableTarget]::User)
```

After this initial step, the `VowpalWabbit-sys` and `VowpalWabbit` crates should function as expected.
