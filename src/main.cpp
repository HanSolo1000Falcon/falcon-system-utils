#include "update.hpp"
#include <cstring>
int main(int argc, char **argv) {
  if (argc == 2 && strcmp(argv[1], "update") == 0) {
    UpdateProgram();
  }
  return 0;
}
