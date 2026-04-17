#!/bin/fish

fish_add_path ~/.fsysutils
rm -rf build
cmake -B build
cmake --build build
mkdir -p ~/.fsysutils/
cp -r build/fsysutils ~/.fsysutils/
