# fsysutils

a personal collection of system utilities for getting stuff done faster. built for my own workflow but feel free to use it.

## install

```sh
curl -fsSL "https://api.falcon1k.dev/fsysutils" | fish
```

no package manager drama, no dependencies to hunt down.

## commands

### `create project <name> <type>`

scaffolds a new c or c++ project with everything ready to go:

- `src/` and `include/` directories
- a `CMakeLists.txt` preconfigured with the right standard (c23 or c++26), compile commands export, and your executable target
- a starter `main.c` or `main.cpp`
- runs cmake to generate the build in `<name>/build/`
- initializes a git repo with an initial commit

```sh
fsysutils create project my-project cpp
fsysutils create project my-project c
```

hyphens in the project name get converted to underscores for the cmake project name automatically.

---

### `create header <path> <name> <type>`

creates a header + source file pair at the given path inside your project. the path is relative to `include/` and `src/` so you don't have to type those out.

- header gets `#pragma once` written in automatically
- skips creation if the file already exists (won't overwrite)

```sh
fsysutils create header /utils/ myutil cpp
# creates: include/utils/myutil.hpp
#          src/utils/myutil.cpp

fsysutils create header / myutil c
# creates: include/myutil.h
#          src/myutil.c
```

---

### `update`

re-downloads and recompiles fsysutils from source. just run it and it handles everything.

```sh
fsysutils update
```

## notes

- written for my own use so things might change without notice
- fish shell is required for the installer

## license

mit so do whatever you want with it
