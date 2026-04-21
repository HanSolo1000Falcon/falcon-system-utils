#!/usr/bin/env fish

fish_add_path ~/.fsysutils
cd ~/falcon-system-utils/
rm -rf build
cmake -B build -G Ninja
cmake --build build
mkdir -p ~/.fsysutils/
mv build/fsysutils ~/.fsysutils/
