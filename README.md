# Wallpaper Renamer (wprn)
This command line tool renames all files in a directory (non-recursively) to a random 10 character string.
It also deletes any duplicate files it finds in that directory.

I use this to have one standard naming convention in my wallpapers folder while removing duplicate wallpapers automatically.

This is probably some of the worst rust code you've ever read. This is my first full Rust project, so it's almost certainly horribly optimized.

# To compile:
Prerequisites:
- Have Cargo installed on your computer
- 
Navigate to the project folder and run
```console
cargo build --release
```

The binary will be in `./target/release/wprn`. You do not need anything else in the release directory.
