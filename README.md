# gw2-cli
Simple, neofetch-like command line tool for displaying Guild Wars 2 stats by using the official API.

![Preview](https://static.p-meier.dev/account.png)

**Note:** This is a learning project I set up to familiarize myself with Rust, since I did not have any prior experience with this tech stack. Feel free to contribute to this project.

## Usage
```bash
Usage: gw2-cli [OPTIONS] [COMMAND]

Commands:
  characters  Character information
  help        Print this message or the help of the given subcommand(s)

Options:
  -l, --lang <LANG>  [default: en]
  -h, --help         Print help information
```

### List characters
```bash
gw2-cli characters list
```

### Character details
```bash
gw2-cli characters <character_name> 
```
![Preview](https://static.p-meier.dev/character_details.png)

# Further ideas / goals
- Localizing static texts
- Achievement points (APs)
- Hall of Monuments points
- In-game currency to collect on the TP
- Number of unlocked character slots
- Available content (PoF, HoT,..)
- Storing the API-key and offering a "login" command
- Different views (PvP, WvW, Rewards...) using `clap` arguments
- Character with the most playtime (incl. profession and specialization)
- Different ascii logos based on the main character or other statistics
