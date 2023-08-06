<p align="center">
  <img width="350" alt="portfolio_view" src="assets/logo.png">
</p>

<br/>


This Rust-based CLI tool reads text and returns it back in bionic reading format
for blazingly fast loading and even faster reading!

Bionic reading is the reading of specially formatted text, allowing for
faster reading. This is possible by strategically highlinting pieces
of text, which tricks the brain of reading without losing content.

**Essentially:** Use this tool to speed up your text reading while using the console

<p align="center">
  <img src="assets/demo.gif" alt="test gif" width="90%" style="box-shadow: 0 0 10px 5px rgba(0, 0, 0, 0.5);">
</p>

## Installation

- **Homebrew**

  ```bash
  brew install ismet55555/things/bieye
  ```

- **Snap Store**

  ```bash
  snap install bieye
  ```

- **Cargo**

  ```bash
  cargo install bieye
  ```

- **Compile From Source**

  ```bash
  git clone git@github.com:ismet55555/bieye.git
  cd bieye
  cargo install --path .
  ```


## Usage Examples

The following are a few simple usage examples for `bieye`

```bash
# Simple text specification
bieye "Hello there, how is your day going?"

# Piping standard out into bieye
cat README.md | bieye
man git | bieye
echo "HELLO! hello hello elo el ..." | bieye

# Add some output options
cat quest.md | bieye --color --dim
```

## CLI Menu

```txt
$ bieye -h

bieye v0.0.0

This CLI tool reads text and returns it back in bionic reading format
for blazingly fast loading and even faster reading!

Usage: bieye [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Capture text from stdin

Options:
  -c, --color    Color highlighted text
  -d, --dim      Dim text not highlighted
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Contribution and Development

Feel free to contribute! Here is a super quick start if you are vaguely familiar with [Rust](https://www.rust-lang.org/tools/install).

```bash
# Setup
git clone git@github.com:ismet55555/bieye.git
cd bieye
git checkout -b my-cool-new-branch
cargo build

# ... work work work ...

# Test run it
cargo run -- --help
echo "Just some testing text" | cargo run --

# Other terminal windows
cargo install --locked bacon
bacon --all-features
```
