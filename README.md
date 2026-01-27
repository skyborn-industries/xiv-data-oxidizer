# XIVData Oxidizer

This is a Rust project that uses [ironworks](https://github.com/ackwell/ironworks) and [EXDSchema](https://github.com/xivdev/EXDSchema) to extract Final Fantasy XIV game data as CSV files. It is a spiritual successor to [SaintCoinach.Cmd](https://github.com/xivapi/SaintCoinach) and provides functionality similar to its `rawexd` command.

Game data is parsed using the schemas provided by EXDSchema which is included as a git submodule. Because the schemas are cloned locally, you can easily make changes to them and extract game data without waiting for the upstream schemas to update.

## Just want the data?

Check out [xiv-data](https://github.com/skyborn-industries/xiv-data).

## Requirements

- Rust 1.87
- A local installation of FFXIV

## Usage

```
git clone --recurse-submodules https://github.com/skyborn-industries/xiv-data-oxidizer
cd xiv-data-oxidizer
# Make sure you run this on patch day to fetch the latest schemas
git submodule update --remote
# Update the following command with your game path as needed
cargo run -- "C:\Program Files (x86)\Square Enix\FINAL FANTASY XIV - A Realm Reborn"
```

---

FINAL FANTASY is a registered trademark of Square Enix Holdings Co., Ltd.

FINAL FANTASY XIV © SQUARE ENIX CO., LTD.
