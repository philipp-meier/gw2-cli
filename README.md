# gw2-cli
Simple, neofetch-like command line tool for displaying Guild Wars 2 stats by using the official API.

![Preview](https://repository-images.githubusercontent.com/543321310/8e131ddc-7e53-4541-98f6-8ce7bebb10a0)

**Note:** This is a learning project I set up to familiarize myself with Rust, since I did not have any prior experience with this tech stack. Feel free to contribute to this project.

## Usage
```bash
Usage: gw2-cli [OPTIONS] --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>  
  -l, --lang <LANG>        [default: en]
  -h, --help               Print help information
```

Currently, the API key will not be stored automatically. I therefore created a `.api_key`-file in the `gw2-cli` folder.
With this file, the application can be executed with the following command: `cargo run -- -a $(cat .api_key) -l en`

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
