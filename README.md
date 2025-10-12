# gw2-cli
[![Build](https://github.com/philipp-meier/gw2-cli/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/philipp-meier/gw2-cli/actions/workflows/rust.yml)
[![MIT License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/philipp-meier/gw2-cli/blob/main/LICENSE)
  
Simple neofetch-like command line tool for displaying Guild Wars 2 stats by using the official API.

![Preview](https://static.p-meier.dev/gw2cli_account.png)

**Note:** This is a learning project I set up to familiarize myself with Rust, since I did not have any prior experience with this tech stack. Feel free to contribute to this project.

## Usage
Set your GW2 API-key to an environment variable named `Gw2Cli_ApiKey`. After that, gw2-cli can be used as described below:  
```bash
Usage: gw2cli [OPTIONS] [COMMAND]

Commands:
  characters  Character information
  help        Print this message or the help of the given subcommand(s)

Options:
  -l, --lang <LANG>  [default: en]
  -h, --help         Print help information
```

### List characters
```bash
gw2cli characters list
```

### Character details
```bash
gw2cli characters <character_name> 
```
![Preview](https://static.p-meier.dev/gw2cli_characters.png)

# Development
## Performance tests

```bash
# sudo apt install linux-tools-generic

# Stats
perf stat -r 10 ./target/release/gw2cli
# List characters
perf stat -r 10 ./target/release/gw2cli characters list
```

## Linter
```bash
cargo clippy
```

# Further ideas / goals
- Localizing static texts
- Achievement points (APs)
- Hall of Monuments points
- In-game currency to collect on the TP
- Number of unlocked character slots
- Available content (PoF, HoT,..)
- Different views (PvP, WvW, Rewards...) using `clap` arguments
- Character with the most playtime (incl. profession and specialization)
- Different ascii logos based on the main character or other statistics
