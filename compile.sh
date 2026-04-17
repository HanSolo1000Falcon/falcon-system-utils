#!/bin/bash

cmake -B build
cmake --build build
mkdir -p ~/.fsysutils/
cp -r build/fsysutils ~/.fsysutils/
rm -rf build
