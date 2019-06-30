# VowpalWabbit-sys-rs
![Crates.io](https://img.shields.io/crates/v/vowpalwabbit-sys.svg)

This crate wraps [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)'s C binding interface. It handles finding the installed VW binaries on the system and linking them into a Rust crate.

See [vowpalwabbit-rs](https://github.com/jackgerrits/vowpalwabbit-rs) for the Rust wrapper around the sys package. This is still a work in progress.

## Install
Currently only supports Linux with VW installed through `make install`.

In order to get the library installed:
1. Clone [VowpalWabbit](https://github.com/VowpalWabbit/vowpal_wabbit)
2. `mkdir build`
3. `cd build`
4. `make -j 8`
5. `make install`

If when trying to run the crate you see:
```
error while loading shared libraries: libvw_c_wrapper.so: cannot open shared object file: No such file or directory
```

You may need to set the library load path:
```sh
LD_LIBRARY_PATH=/usr/local/lib
export LD_LIBRARY_PATH
```