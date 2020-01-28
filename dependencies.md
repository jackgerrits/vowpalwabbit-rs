# Dependencies
Dependencies for VowpalWabbit are packaged with the crate and are built from source. In order to get a standalone, from source build working, several repos had to be forked.

- `external/vowpal_wabbit`
	- Forked in order to bundle dependencies in the source tree. The CMake build definition had the find_package parts remove and the external directory added
- `external/vowpal_wabbit/external/zlib`
	- The CMakeLists.txt checked into Zlib includes a file rename in the source dir that breaks the docs.rs source dir read only build
- `external/vowpal_wabbit/external/boost-cmake`
	- The experimental[git support](https://github.com/Orphis/boost-cmake/tree/git_support)branch is being used. Instead of allowing it to use `FetchContent`, there is a custom fork of Boost checked in. This repo is forked to remove the `FetchContent` usage and bring the `cmake_minimum_required` down to `3.10` in order to support docs.rs
- `external/vowpal_wabbit/external/boost`
	- `boost-cmake` enumerates the `libs` directory. To reduce crate size and configure time all of the unused libs had to be removed from this dir
	- See [below](#Boost-dependency) for the Boost dependencies used and how to update the fork if needed


## Boost dependencies
Current Boost dependencies:
- align
- any
- array
- assert
- bind
- concept_check
- config
- container
- container_hash
- core
- detail
- function
- headers
- integer
- iterator
- lexical_cast
- math
- move
- mpl
- numeric_conversion
- predef
- preprocessor
- program_options
- range
- smart_ptr
- static_assert
- static_cast
- throw_exception
- tokenizer
- type_index
- type_traits
- utility

### How to update Boost dependencies
Remove all non-included submodules:
```sh
cd libs
rm -rf !(align|any|array|assert|bind|concept_check|config|container|container_hash|core|detail|function|headers|integer|iterator|lexical_cast|math|move|mpl|numeric|predef|preprocessor|program_options|range|smart_ptr|static_assert|static_cast|throw_exception|tokenizer|type_index|type_traits|utility|*html|*.txt)
cd numeric
rm -rf !(conversion|*html|*.txt)
cd ../..
```

Update `.gitmodules` and change the relative path to an absolute path:
```
[submodule "config"]
	path = libs/config
	url = https://github.com/boostorg/config.git
	fetchRecurseSubmodules = on-demand
```