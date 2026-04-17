cmake_minimum_required(VERSION 4.1)

project(falcon_system_utils LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_SCAN_FOR_MODULES OFF)

add_executable(falcon_system_utils src/main.cpp)
target_include_directories(falcon_system_utils PRIVATE include)

