#include "create-header.hpp"
#include <filesystem>
#include <fstream>
#include <iostream>

using namespace std::filesystem;

int CreateHeader(const std::string &path, const std::string &name,
                 const std::string &type) {
  auto trim = [](std::string s) {
    s.erase(0, s.find_first_not_of('/'));
    s.erase(s.find_last_not_of('/') + 1);
    return s;
  };
  std::string cleanPath = trim(path);

  std::string headerExt, sourceExt;
  if (type == "c") {
    headerExt = "h";
    sourceExt = "c";
  } else if (type == "cpp") {
    headerExt = "hpp";
    sourceExt = "cpp";
  } else {
    std::cerr << "invalid type: " << type << " (use 'c' or 'cpp')\n";
    return 1;
  }

  class path includeDir = "./include/" + cleanPath;
  class path srcDir = "./src/" + cleanPath;
  create_directories(includeDir);
  create_directories(srcDir);

  class path headerFile = includeDir / (name + "." + headerExt);
  class path sourceFile = srcDir / (name + "." + sourceExt);

  if (!exists(headerFile)) {
    std::ofstream(headerFile) << "#pragma once\n";
  } else {
    std::cout << "header already exists: " << headerFile << "\n";
  }

  if (!exists(sourceFile)) {
    std::ofstream ofs(sourceFile);
  } else {
    std::cout << "source already exists: " << sourceFile << "\n";
  }

  std::cout << "created:\n  " << headerFile << "\n  " << sourceFile << "\n";
  return 0;
}
