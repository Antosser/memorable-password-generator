# Memorizable Password Generator
Create memorizable passwords easily using this CLI

## Installation
### Windows
Download the executable from the [latest release](https://github.com/Antosser/memorizable-password-generator/releases/latest) and put it somewhere in your path or in your working directory

### Linux / Compiling yourself
1. Download Rust
2. Run `cargo build -r`
3. Binary is in `./target/release/`
4. Put it in your path or working directory

## Usage
```
Create memorable passwords easily using this CLI

Usage: mpgen [OPTIONS]

Options:
  -s, --start <START>      The type of letter at the beginning of the password [default: consonant] [possible values: vowel, consonant]
  -l, --letters <LETTERS>  Amount of letters [default: 7]
  -n, --numbers <NUMBERS>  How many pairs of numbers to add at the end [default: 1]
  -S, --symbols <SYMBOLS>  How many symbols to add at the end [default: 0]
  -h, --help               Print help
  -V, --version            Print version
```
