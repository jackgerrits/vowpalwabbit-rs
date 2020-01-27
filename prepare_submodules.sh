#!/bin/bash
git submodule update --init external/vowpal_wabbit
cd external/vowpal_wabbit
git submodule update --init rapidjson
git submodule update --init external/boost
git submodule update --init external/boost-cmake
git submodule update --init external/zlib
cd external/boost
git submodule update --init libs/*
cd ../../../../
