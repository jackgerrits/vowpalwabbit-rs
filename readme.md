# VowpalWabbit-rs
[![build](https://github.com/jackgerrits/vowpalwabbit-rs/workflows/build/badge.svg?branch=master)](https://github.com/jackgerrits/vowpalwabbit-rs/actions?query=workflow%3Abuild)

| Crate    | <!-- -->    |
|-------------|-------------|
| `vowpalwabbit-sys` | [![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)](https://crates.io/crates/vowpalwabbit-sys) [![Docs](https://docs.rs/vowpalwabbit-sys/badge.svg)](https://docs.rs/vowpalwabbit-sys) |
| `vowpalwabbit` | ... |

This repo provides:
- DLL which wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)
- Sys crate providing access to this DLL's API
- Crate providing a safe rust interface to VowpalWabbit as well as other utilities

## Versioning
The major and minor versions of the sys crate track that of the native VW library that is wraps. The patch version, though, may be out of sync due to the need to patch the crate out of sync with the native dependency. Starting at version `8.8.1+vw-v8.8.0` you can determine the version of Vowpal Wabbit that it wraps by looking at the associated SemVer metadata. In this case it is `vw-v8.8.0` indicating the wrapped Vowpal Wabbit version is `8.8.0`.

The non-sys crate simply depends on some version of the sys crate which can be used to determine the VowpalWabbit version wrapped. The crate itself is versioned monotonically with potentially several releases per native VW release.

## Setup

All versions prior to `9.3.0` used a different scheme for binding and interfacing with the VowpalWabbit library. Therefore, these instruction apply for `9.3.0` onwards.

The `VowpalWabbit-sys` crate searches for a library called `vw_rs_bindings`. This needs to be findable on your system in order to use the bindings.

### Build

#### Ubuntu

```sh
git submodule update --init
cd binding
cmake --preset=vcpkg
cmake --build --preset=vcpkg
```

#### MacOS

```sh
git submodule update --init
cd binding
cmake --preset=vcpkg
cmake --build --preset=vcpkg
```

#### Windows

The following MUST be run in an x64 developer tools prompt. Not using 64 bit will cause issues later when the DLL is loaded.

```powershell
git submodule update --init
cd binding
cmake --preset=vcpkg-windows
cmake --build --preset=vcpkg-windows
```

### Make findable in build dir

Set environment variable `VW_RS_BINDING_HOME` to the `binding/build` directory.

On Windows add the `binding/build` directory to the `path`.
On MacOS or Ubuntu set `LD_LIBRARY_PATH` to `binding/build`.

### Install (optional)

#### Ubuntu
```sh
sudo cmake --install build --strip

# Make sure this is set when running cargo
export LD_LIBRARY_PATH=/lib:/usr/lib:/usr/local/lib
```

#### MacOS
```sh
sudo cmake --install build --strip
```

#### Windows

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
