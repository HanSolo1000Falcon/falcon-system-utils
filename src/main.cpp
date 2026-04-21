#include "create-commands.hpp"
#include <cstring>
int main(int argc, char **argv) {
  if (strcmp(argv[1], "create") == 0) {
    return CreateCommands::CallCreate(argc, argv);
  }

  return 0;
}
