# Wallpaper Renamer (wprn)
This command line tool renames all files in a directory (non-recursively) to a random 10 character string.
It also deletes any duplicate files it finds in that directory.

Used to have one standard naming convention in my wallpapers folder while removing duplicate wallpapers automatically.

# To compile:
Prerequisites:
- Have Rust and Cargo installed on your computer

1. Clone this repo using 
```console 
git clone https://github.com/opDavi1/wprn --depth=1
```

2. Navigate to the project folder and run
```console
cargo build --release
```

3. The binary will be in `./target/release/wprn`. You do not need anything else in the release directory.
