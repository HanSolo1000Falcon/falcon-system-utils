#include "update.hpp"
#include <cstdlib>
#include <filesystem>
#include <string>
#include <system_error>

void UpdateProgram() {
  std::string home = getenv("HOME");
  std::filesystem::path repoPath = home + "/falcon-system-utils";

  if (std::filesystem::exists(repoPath)) {
    std::error_code error;
    std::filesystem::remove_all(repoPath, error);
  }

  std::string cloneCmd =
      "git clone "
      "\"https://github.com/HanSolo1000Falcon/falcon-system-utils.git\" " +
      repoPath.string();
  system(cloneCmd.c_str());

  std::string chmodCmd = "chmod +x " + repoPath.string() + "/compile.fish";
  system(chmodCmd.c_str());

  std::string runCmd = repoPath.string() + "/compile.fish";
  system(runCmd.c_str());

  std::error_code error;
  std::filesystem::remove_all(repoPath, error);
}
