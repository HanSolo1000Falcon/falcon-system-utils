#include "dotfiles-commands.hpp"
#include <cstdlib>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <ios>
#include <iostream>
#include <string_view>
constexpr std::string_view INVALID_USAGE =
    "Invalid usage.\nUsage: fsysutils dotfiles <subcommand> "
    "*\n\nAvailable subcommands:\n  "
    "fetch\n  add <name>\n  push\n";

int DotfilesCommands::CallDotfiles(int argc, char **argv) {
  if (argc < 3) {
    std::cout << INVALID_USAGE;
    return 1;
  }

  if (strcmp(argv[2], "fetch") == 0) {
    return FetchDotfiles(argc, argv);
  } else if (strcmp(argv[2], "add") == 0) {
    return AddDotfiles(argc, argv);
  } else if (strcmp(argv[2], "push") == 0) {
    return PushDotfiles(argc, argv);
  }

  std::cout << INVALID_USAGE;
  return 1;
}

int DotfilesCommands::FetchDotfiles(int argc, char **argv) {
  const auto configDirectory =
      std::filesystem::path(getenv("HOME")) / ".config";
  const auto gitDirectory = configDirectory / ".git";
  const auto remote = "https://github.com/HanSolo1000Falcon/.config.git";

  if (!std::filesystem::exists(gitDirectory)) {
    system(("git -C " + configDirectory.string() + " init").c_str());
    system(("git -C " + configDirectory.string() + " remote add origin " + remote).c_str());
    system(("git -C " + configDirectory.string() + " fetch").c_str());
    system(("git -C " + configDirectory.string() +
            " checkout -b main --track origin/main")
               .c_str());
  }

  system(("git -C " + configDirectory.string() + " fetch origin").c_str());
  system(("git -C " + configDirectory.string() + " reset --hard origin/main")
             .c_str());
  return 0;
}

int DotfilesCommands::AddDotfiles(int argc, char **argv) {
  if (argc < 4) {
    std::cout << "Invalid usage.\n";
    return 1;
  }

  const auto configDirectory =
      std::filesystem::path(getenv("HOME")) / ".config" / "fsysutils";

  std::error_code caughtError;
  if (!std::filesystem::exists(configDirectory, caughtError)) {
    std::filesystem::create_directory(configDirectory);
  }

  std::ofstream gitAddFile(configDirectory / "git_add.fsysutil",
                           std::ios_base::app | std::ios_base::out);
  gitAddFile << " " << argv[3];
  return 0;
}

int DotfilesCommands::PushDotfiles(int argc, char **argv) {
  const auto configDirectory =
      std::filesystem::path(getenv("HOME")) / ".config";
  std::string gitAdds;
  std::ifstream gitAddFile(configDirectory / "fsysutils" / "git_add.fsysutil");
  std::getline(gitAddFile, gitAdds);

  system(("git -C " + configDirectory.string() + " add " + gitAdds).c_str());
  system(("git -C " + configDirectory.string() +
          " commit -m \"Commited with fsysutils.\"")
             .c_str());
  system(("git -C " + configDirectory.string() + " push origin main").c_str());
  return 0;
}
