# bender_bouncer

The bouncer has two responsibilities:
- checking if a file is a valid .blend file
- collecting basic data about the framerange
It is a Rust implementation of *blender_render_info.py* that comes bundled with python

bender-bouncer only reads the binary data of the .blend file and therefore
doesn't need a blender executable. This is also much faster than opening the
.blend with Blender

### bender bouncer consists of two parts
- a **rust library** exposing the checking functionality so it can be used elsewhere
- a small CLI tool called **bender-bouncer-cli**, that allows for manual or scripted checking

### Library Usage
The library can be loaded via public git mirror:
   bender_bouncer = { git = "https://github.com/atoav/bender-bouncer.git" }

### Documentation
To view the documentation on the various functions run
    cargo doc --no-deps --open

### Installation (CLI tool)
1. Make sure you have rust and cargo installed (easiest with [rustup](http://rustup.rs))
2. Clone the repo via `git clone` and go into the repo with `cd bender_bouncer`
3. run `cargo build --release`
4. copy the compiled binary `./target/release/bender-bouncer-cli` wherever you like

