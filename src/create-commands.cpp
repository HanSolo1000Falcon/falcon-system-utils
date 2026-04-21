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

int CreateCommands::CreateProject(int argc, char **argv) { return 0; }
