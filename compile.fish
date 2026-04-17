#!/bin/fish

fish_add_path ~/.fsysutils
cmake -B build
cmake --build build
mkdir -p ~/.fsysutils/
cp -r build/fsysutils ~/.fsysutils/
rm -rf build
