#include "create-commands.hpp"
#include "dotfiles-commands.hpp"
#include <cstring>
#include <iostream>
int main(int argc, char **argv) {
  if (argc < 2) {
    std::cout
        << "Invalid usage; try 'fsysutils help' for usage instructions.\n";
    return 1;
  }

  if (strcmp(argv[1], "help") == 0) {
    std::cout
        << "fsysutils help wizard.\n"
        << "Usage: fsysutils <command> <subcommand>\n\n"
        << "Available commands:\n"
        << "  update\n"
        << "  create\n"
        << "  dotfiles\n"
        << "\n"
        << "Run any of them to get their individual usage instructions.\n";
    return 0;
  }

  if (strcmp(argv[1], "update") == 0) {
    std::cout << "Updating fsysutils...\n";
    system("curl -fsSL \"https://api.falcon1k.dev/fsysutils\" | fish");
    std::cout << "Done updating.\n";
  }

  if (strcmp(argv[1], "create") == 0) {
    return CreateCommands::CallCreate(argc, argv);
  }

  if (strcmp(argv[1], "dotfiles") == 0) {
    return DotfilesCommands::CallDotfiles(argc, argv);
  }

  std::cout << "Invalid usage; try 'fsysutils help' for usage instructions.\n";
  return 0;
}
