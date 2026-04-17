#include "update.hpp"
#include <cstring>
int main(int argc, char **argv) {
  if (argc == 2 && strcmp(argv[1], "update")) {
    UpdateProgram();
  }
  return 0;
}
