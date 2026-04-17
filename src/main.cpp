#include "update.hpp"
#include <cstring>
#include <iostream>
int main(int argc, char **argv) {
  if (argc == 2 && strcmp(argv[1], "update") == 0) {
    std::cout << "Updating program\n\n\n";

    UpdateProgram();
    return 0;
  }

  std::cout << "fsysutils help wizard!\n"
            << " usage: fsysutils <command> <subcommand (possibly exists, "
               "check command usage)> <args>\n\n"
            << " availabe commands:\n"
            << "   update\n"
            << "     you guessed it, the program updates itself\n";

  return 0;
}
