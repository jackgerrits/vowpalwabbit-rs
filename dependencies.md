Dependencies for VowpalWabbit are packaged with the crate and are built from source. ZLib is simply bundled as a submodule and included with CMake. Boost, however, is a bit more complex. The experimental[git support](https://github.com/Orphis/boost-cmake/tree/git_support) in [Boost-CMake](https://github.com/Orphis/boost-cmake) is being used. Instead of allowing it to use `FetchContent`, there is a custom fork of Boost checked in. It is forked because using a partial checkout of the Boost modular repo did not play well with Boost-CMake.

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

### To update Boost dependencies:
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