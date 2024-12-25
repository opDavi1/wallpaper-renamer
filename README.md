# Wallpaper Renamer (wprn)
This command line tool renames all files in a directory (non-recursively) to a random 10 character string.
It also deletes any duplicate files it finds in that directory.

I use this to have one standard naming convention in my wallpapers folder while removing duplicate wallpapers automatically.

This is probably some of the worst rust code you've ever read. This is my first full Rust project, so it's almost certainly horribly optimized.
Feel free to contribute any changes or additions you want. Just make a PR. I'm certain that almost any change from someone who actually knows rust would make this project 10 times better

# Installation
## Linux
Go to the latest release and download the binary or the PKGBUILD if you are on Arch.
If you would like to compile from source, then see below. The PKGBUILD already builds from source.

## Windows & Mac
You will have to compile from source, see below.



# Compilation
To compile from source, first make sure that cargo is installed and download the source code. Then simply run:
```console
$ cargo build --release
```
The binary will be in `target/release/`.

