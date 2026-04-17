#include "update.hpp"
#include <cstdlib>
#include <filesystem>
#include <system_error>

void UpdateProgram() {
  if (std::filesystem::exists("~/falcon-system-utils")) {
    std::error_code error;
    std::filesystem::remove_all("~/falcon-system-utils", error);
  }

  system("git clone "
         "\"https://github.com/HanSolo1000Falcon/falcon-system-utils.git\" "
         "~/falcon-system-utils");
  system("chmod +x ~/falcon-system-utils/compile.fish");
  system("~/falcon-system-utils/compile.fish");
  std::error_code error;
  std::filesystem::remove_all("~/falcon-system-utils", error);
}
