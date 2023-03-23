# Memorizable Password Generator
Create memorizable passwords easily using this CLI

## Installation
### Windows
Download the executable from the latest release and put it somewhere in your path or in your working directory

### Linux / Compiling yourself
1. Download Rust
2. Run `cargo build -r`
3. Binary is in `./target/release/`
4. Put it in your path or working directory

## Usage
```
Create memorizable passwords easily using this CLI

Usage: mpg [OPTIONS]

Options:
  -s, --symbol                       Whether to add a random symbol at the end
  -w, --words <WORDS>                The amount of words [default: 2]
  -n, --numberspairs <NUMBERSPAIRS>  The amount of number pairs [default: 1]
  -h, --help                         Print help
  -V, --version                      Print version
```
