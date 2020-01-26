#!/bin/bash
shopt -s extglob
git submodule update --init external/vowpal_wabbit
cd external/vowpal_wabbit
git submodule update --init rapidjson
git submodule update --init external/boost
git submodule update --init external/boost-cmake
git submodule update --init external/zlib
cd external/boost/libs
rm -rf !(config|detail|range|iterator|headers|function|lexical_cast|any|type_traits|static_cast|core|program_options|math)
git submodule update --init *
cd ../../../../
