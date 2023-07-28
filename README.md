# Memorizable Password Generator
Create memorizable passwords easily using this CLI

## Installation
### Compiling and installing from source
1. Get [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Run `cargo install mpgen`
3. Run `mpgen`

### Windows (pre-compiled)
Download the executable from the [latest release](https://github.com/Antosser/memorizable-password-generator/releases/latest) and put it somewhere in your path or in your working directory

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

## Examples
```
~ mpg
konamit67

~ mpg -s vowel
uxojaca45

~ mpg -s vowel -l 4
ubal12

~ mpg -s vowel -l 4 -n 2
emib7812

~ mpg -s vowel -l 4 -n 2 -S 3
ecun2345=%&
```
