#include "dotfiles.hpp"
#include <cstdlib>
#include <filesystem>
#include <string>

namespace fs = std::filesystem;

int Dotfiles() {
  const std::string configDir = std::string(getenv("HOME")) + "/.config";
  const std::string gitDir = configDir + "/.git";
  const std::string remote = "https://github.com/HanSolo1000Falcon/.config.git";

  if (!fs::exists(gitDir)) {
    std::system(("git -C " + configDir + " init").c_str());
    std::system(
        ("git -C " + configDir + " remote add origin " + remote).c_str());
    std::system(("git -C " + configDir + " fetch").c_str());
    std::system(
        ("git -C " + configDir + " checkout -b main --track origin/main")
            .c_str());
  }

  std::system(("git -C " + configDir + " pull origin main").c_str());
  return 0;
}
