cmake_minimum_required(VERSION 4.1)

project(falcon_system_utils LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_SCAN_FOR_MODULES OFF)

add_executable(fsysutils src/main.cpp
            src/update.cpp
            src/create-header.cpp
            src/create-project.cpp
            src/dotfiles.cpp
          )
target_include_directories(fsysutils PRIVATE include)

