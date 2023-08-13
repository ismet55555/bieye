//////////////////////////////////////////////////////////////////////////////////
// This file is used to test code snippets that are not used in the main program
//////////////////////////////////////////////////////////////////////////////////


//////////////////////////////////////////////////////////////////////////////////
// Attempt to solve hanging stdin read
//////////////////////////////////////////////////////////////////////////////////
use os_pipe::pipe;
use std::io::prelude::*;

// Check if there are any command-line arguments
let has_args = std::env::args().count() > 1;
println!("HAS ARGS: {}", has_args);

// Create a non-blocking pipe for reading from stdin
let (reader, _) = pipe()?;

// Read from stdin if available
let mut input_from_stdin = String::new();
let mut stdin = BufReader::new(reader);
stdin.read_to_string(&mut input_from_stdin)?;

println!("PIPED STDIN: {}", input_from_stdin);

// If there's nothing read from stdin, return an error
if input_from_stdin.is_empty() {
    return Err(eyre!("No text provided via command-line argument or stdin"));
}



//////////////////////////////////////////////////////////////////////////////////
/// Remove spaces
//////////////////////////////////////////////////////////////////////////////////
        let words: Vec<&str> = re_words
            .find_iter(self.text_input.as_str())
            .map(|mat| mat.as_str())
            .filter(|word| !word.trim().is_empty())  // REMOVING SPACES
            .collect();



//////////////////////////////////////////////////////////////////////////////////
/// Loading stdin
//////////////////////////////////////////////////////////////////////////////////
use std::{io, io::prelude::*};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
    for line in io::stdin().lock().lines() {
        println!("length = {}", line?.len());
    }
    Ok(())
}



//////////////////////////////////////////////////////////////////////////////////
/// Pick highlight color
//////////////////////////////////////////////////////////////////////////////////
    /// Color of highlighted text (default: yellow)
        // FIXME: This is currently not working
        // FIXME: Parse in Bieye struct and check
    #[clap(
        short = 'y',
        long,
        value_name = "STRING",
        required = false,
    )]
    pub color_name: Option<String>,





//////////////////////////////////////////////////////////////////////////////////
/// GitHub Actions Release Workflow
//////////////////////////////////////////////////////////////////////////////////
---
name: Release to GitHub

on:
  push:
    tags:
      - v1.*

jobs:
  create-release:
    name: ${{ matrix.name }}
    runs-on: ${{ matrix.os }}
    needs: Testing  # Test GitHub Workflow

    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            name: Linux Binary 64-Bit
            target: x86_64-unknown-linux-musl

          - os: macos-latest
            name: MacOS Binary 64-Bit
            target: x86_64-apple-darwin
            target2: aarch64-apple-darwin
            env:
              MACOSX_DEPLOYMENT_TARGET: 10.7

          - os: windows-latest
            name: Windows Binary 64-Bit
            target: x86_64-pc-windows-msvc

    steps:
      - name: Check out repository
        uses: actions/checkout@v3

      - name: Add rustup default target
        run: rustup target add ${{ matrix.target }}

      - name: Add rustup Apple ARM64 target
        if: ${{ matrix.os == 'macos-latest' }}
        run: rustup target add ${{ matrix.target2 }}

      - name: Build default target in release mode
        run: cargo build --release --target ${{ matrix.target }} --locked

      - name: Build Apple ARM64 target in release mode
        if: ${{ matrix.os == 'macos-latest' }}
        run: cargo build --release --target ${{ matrix.target2 }} --locked

      - name: Get latest release version number
        id: get_version
        uses: battila7/get-version-action@v2

      - name: Create zip file on Windows
        if: ${{ matrix.os == 'windows-latest' }}
        run: |
          choco install zip
          cd target/${{ matrix.target }}/release
          zip bieye-${{ steps.get_version.outputs.version }}-${{ matrix.target }}.zip bieye.exe
          cd ../../..

      - name: Create tar.gz file on macOS
        if: ${{ matrix.os == 'macos-latest' }}
        run: |
          chmod +x target/${{ matrix.target }}/release/bieye
          tar -zcf target/${{ matrix.target }}/release/bieye-${{ steps.get_version.outputs.version }}-${{ matrix.target }}.tar.gz -C target/${{ matrix.target }}/release bieye
          chmod +x target/${{ matrix.target2 }}/release/bieye
          tar -zcf target/${{ matrix.target2 }}/release/bieye-${{ steps.get_version.outputs.version }}-${{ matrix.target2 }}.tar.gz -C target/${{ matrix.target2 }}/release bieye

      - name: Create tar.gz file on Linux
        if: ${{ matrix.os == 'ubuntu-latest' }}
        run: |
          chmod +x target/${{ matrix.target }}/release/bieye
          tar -zcf target/${{ matrix.target }}/release/bieye-${{ steps.get_version.outputs.version }}-${{ matrix.target }}.tar.gz -C target/${{ matrix.target }}/release bieye

      - name: Upload release and assets to GitHub
        uses: svenstaro/upload-release-action@v2
        with:
          repo_token: ${{ secrets.GITHUB_TOKEN }}
          tag: ${{ github.ref }}
          release_name: bieye ${{ steps.get_version.outputs.version }}
          file_glob: true
          file: target/*/release/bieye-${{ steps.get_version.outputs.version }}-*.{zip,tar.gz}

      - name: Upload release to crates.io
        uses: katyo/publish-crates@v2
        if: ${{ matrix.os == 'ubuntu-latest' }}
        with:
          registry-token: ${{ secrets.CARGO_REGISTRY_TOKEN }}
