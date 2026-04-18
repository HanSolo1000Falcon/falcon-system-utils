#include "create-project.hpp"
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <system_error>

void ReplaceAll(std::string &str, const std::string &from,
                const std::string &to) {
  size_t pos = 0;
  while ((pos = str.find(from, pos)) != std::string::npos) {
    str.replace(pos, from.length(), to);
    pos += to.length();
  }
}

int CreateProject(const std::string &name, const std::string &type) {
  std::error_code error;
  if (std::filesystem::exists("./" + name, error)) {
    std::cout << "there seems to already be a project with that name here, "
                 "aborting...\n";
    return 1;
  }

  auto formattedName = name;
  ReplaceAll(formattedName, "-", "_");

  std::filesystem::create_directory(name);
  std::filesystem::create_directory(std::filesystem::path(name) / "src");
  std::filesystem::create_directory(std::filesystem::path(name) / "include");

  if (type == "c") {
    std::ofstream(std::filesystem::path(name) / "src" / "main.c")
        << "#include <stdio.h>\n\nint main(void) {\n  return 0;\n}";
    std::ofstream(std::filesystem::path(name) / "CMakeLists.txt")
        << "cmake_minimum_required(VERSION 4.1)\n\nproject(" + formattedName +
               " LANGUAGES C)\n\nset(CMAKE_C_STANDARD "
               "23)\nset(CMAKE_C_STANDARD_REQUIRED "
               "ON)\nset(CMAKE_EXPORT_COMPILE_COMMANDS "
               "ON)\nset(CMAKE_CXX_SCAN_FOR_MODULES OFF)\n\nadd_executable(" +
               formattedName + " src/main.c)\ntarget_include_directories(" +
               formattedName + " PRIVATE include)";
    system(std::format("cmake -S {} -B {}/build", name, name).c_str());
  } else if (type == "cpp") {
    std::ofstream(std::filesystem::path(name) / "src" / "main.cpp")
        << "int main() {\n  return 0;\n}";
    std::ofstream(std::filesystem::path(name) / "CMakeLists.txt")
        << "cmake_minimum_required(VERSION 4.1)\n\nproject(" + formattedName +
               " LANGUAGES CXX)\n\nset(CMAKE_CXX_STANDARD "
               "26)\nset(CMAKE_CXX_STANDARD_REQUIRED "
               "ON)\nset(CMAKE_EXPORT_COMPILE_COMMANDS "
               "ON)\nset(CMAKE_CXX_SCAN_FOR_MODULES OFF)\n\nadd_executable(" +
               formattedName + " src/main.cpp)\ntarget_include_directories(" +
               formattedName + " PRIVATE include)";
    system(std::format("cmake -S {} -B {}/build", name, name).c_str());
  } else {
    std::cout << "invalid type: " << type << " (use 'c' or 'cpp')";
    return 1;
  }

  system(std::format("git init ./{}", name).c_str());
  system(std::format("git -C ./{} add .", name).c_str());
  system(std::format("git -C ./{} commit -m \"Initial commit\"", name).c_str());

  std::cout << "created project!\n";
  return 0;
}
