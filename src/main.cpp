#include "create-header.hpp"
#include "update.hpp"
#include <cstring>
#include <iostream>

int main(int argc, char **argv) {
  if (argc == 2 && strcmp(argv[1], "update") == 0) {
    std::cout << "updating program\n\n";
    UpdateProgram();
    return 0;
  }

  if (argc == 6 && strcmp(argv[1], "create") == 0 &&
      strcmp(argv[2], "header") == 0) {
    std::cout << "creating header\n\n";
    return CreateHeader(std::string(argv[3]), std::string(argv[4]),
                        std::string(argv[5]));
  }

  std::cout
      << "fsysutils help wizard!\n"
      << " usage: fsysutils <command> <subcommand (possibly exists, "
         "check command usage)> <args>\n\n"
      << " availabe commands:\n"
      << "   update\n"
      << "     you guessed it, the program updates itself\n\n"
      << "   create\n"
      << "     header\n"
      << "      args: path (the path to the file, i.e if i chose '/test/' it "
         "would create the header in include/test and the source file in "
         "src/test), name (the name of the header), type (c|cpp)\n"
      << "      creates an already set up header source file combo with the "
         "specified name and path, the header has '#pragma once' already "
         "written to it";

  return 0;
}
