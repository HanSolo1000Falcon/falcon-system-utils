#!/bin/fish

fish_add_path ~/.fsysutils
rm -rf ~/falcon-system-utils/build
cmake -B ~/falcon-system-utils/build
cmake --build ~/falcon-system-utils/build
mkdir -p ~/.fsysutils/
mv ~/falcon-system-utils/build/fsysutils ~/.fsysutils/
