#include "create-commands.hpp"
#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
int CreateCommands::CallCreate(int argc, char **argv) {
  if (argc >= 3) {
    if (strcmp(argv[2], "header") == 0) {
      return CreateHeader(argc, argv);
    } else if (strcmp(argv[2], "project") == 0) {
      return CreateProject(argc, argv);
    }

    std::cout << "Invalid usage.\n"
              << "Usage: fsysutils create <subcommand>*\n\n"
              << "Subcommands:\n"
              << "  header <subdirectory> <name> <type(c|cpp)>\n"
              << "  project <name> <type(c|cpp)>\n";
  }

  std::cout << "Invalid usage.\n"
            << "Usage: fsysutils create <subcommand>*\n\n"
            << "Subcommands:\n"
            << "  header <subdirectory> <name> <type(c|cpp)>\n"
            << "  project <name> <type(c|cpp)>\n";
  return 0;
}

int CreateCommands::CreateHeader(int argc, char **argv) {
  if (argc != 6) {
    std::cout << "Invalid usage\n";
    return 1;
  }

  auto trim = [](std::string str) {
    str.erase(0, str.find_first_not_of('/'));
    str.erase(str.find_last_not_of('/') + 1);
    return str;
  };

  const auto cleanPath = trim(argv[3]);

  std::string headerExtension, sourceExtension;
  if (strcmp(argv[5], "c") == 0) {
    headerExtension = "h";
    sourceExtension = "c";
  } else if (strcmp(argv[5], "cpp") == 0) {
    headerExtension = "hpp";
    sourceExtension = "cpp";
  } else {
    std::cout << "Invalid usage\n";
    return 1;
  }

  class std::filesystem::path includeDirectory = "./include/" + cleanPath;
  class std::filesystem::path sourceDirectory = "./src/" + cleanPath;

  std::filesystem::create_directory(includeDirectory);
  std::filesystem::create_directory(sourceDirectory);

  class std::filesystem::path headerFile =
      includeDirectory / (std::string(argv[4]) + "." + headerExtension);
  class std::filesystem::path sourceFile =
      sourceDirectory / (std::string(argv[4]) + "." + sourceExtension);

  if (!std::filesystem::exists(headerFile)) {
    std::ofstream(headerFile) << "#pragma once\n";
  } else {
    std::cout << "Header already exists.\n";
  }

  if (std::filesystem::exists(sourceFile)) {
    std::ofstream foo(sourceFile);
  } else {
    std::cout << "Source file already exists\n";
  }

  return 0;
}

int CreateCommands::CreateProject(int argc, char **argv) {
  if (argc != 5) {
    std::cout << "Invalid usage\n";
    return 1;
  }

  std::error_code caughtError;
  if (std::filesystem::exists("./" + std::string(argv[3]), caughtError)) {
    std::cout
        << "A project with the specified name already exists, aborting...\n";
    return 1;
  }

  auto formattedName = std::string(argv[3]);
  size_t formattingPosition = 0;
  while ((formattingPosition = formattedName.find("-", formattingPosition)) !=
         std::string::npos) {
    formattedName.replace(formattingPosition, 1, "_");
    ++formattingPosition;
  }

  std::filesystem::create_directory(argv[3]);
  std::filesystem::create_directory(std::filesystem::path(argv[3]) / "src");
  std::filesystem::create_directory(std::filesystem::path(argv[3]) / "include");

  if (strcmp(argv[4], "c") == 0) {
    std::ofstream(std::filesystem::path(argv[3]) / "src" / "main.c")
        << "#include <stdio.h>\n\nint main(void) {\n  return 0;\n}";
    std::ofstream(std::filesystem::path(argv[3]) / "CMakeLists.txt")
        << "cmake_minimum_required(VERSION 4.1)\n\nproject(" + formattedName +
               " LANGUAGES C)\n\nset(CMAKE_C_STANDARD "
               "23)\nset(CMAKE_C_STANDARD_REQUIRED "
               "ON)\nset(CMAKE_EXPORT_COMPILE_COMMANDS "
               "ON)\nset(CMAKE_CXX_SCAN_FOR_MODULES OFF)\n\nadd_executable(" +
               formattedName + " src/main.c)\ntarget_include_directories(" +
               formattedName + " PRIVATE include)";
    system(std::format("cmake -S {} -B {}/build -G Ninja", argv[3], argv[3])
               .c_str());
  } else if (strcmp(argv[4], "cpp") == 0) {
    std::ofstream(std::filesystem::path(argv[3]) / "src" / "main.cpp")
        << "int main() {\n  return 0;\n}";
    std::ofstream(std::filesystem::path(argv[3]) / "CMakeLists.txt")
        << "cmake_minimum_required(VERSION 4.1)\n\nproject(" + formattedName +
               " LANGUAGES CXX)\n\nset(CMAKE_CXX_STANDARD "
               "26)\nset(CMAKE_CXX_STANDARD_REQUIRED "
               "ON)\nset(CMAKE_EXPORT_COMPILE_COMMANDS "
               "ON)\nset(CMAKE_CXX_SCAN_FOR_MODULES OFF)\n\nadd_executable(" +
               formattedName + " src/main.cpp)\ntarget_include_directories(" +
               formattedName + " PRIVATE include)";
    system(std::format("cmake -S {} -B {}/build -G Ninja", argv[3], argv[3])
               .c_str());
  } else {
    std::cout << "Invalid usage.\n";
    return 1;
  }

  system(std::format("git init ./{}", argv[3]).c_str());
  system(std::format("git -C ./{} add .", argv[3]).c_str());
  system(
      std::format("git -C ./{} commit -m \"Initial commit\"", argv[3]).c_str());

  return 0;
}
