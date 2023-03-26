use clap::{Parser, ValueEnum};
use rand::Rng;

#[derive(ValueEnum, Clone, Debug, strum_macros::Display, PartialEq)]
enum LetterType {
    Vowel,
    Consonant,
}
impl LetterType {
    pub fn other(self) -> Self {
        match self {
            Self::Vowel => Self::Consonant,
            Self::Consonant => Self::Vowel,
        }
    }
}

/// Create memorizable passwords easily using this CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The type of letter at the beginning of the password
    #[arg(short, long, default_value = "consonant")]
    start: LetterType,

    /// The type of letter at the end of the password
    #[arg(short, long, default_value = "vowel")]
    end: LetterType,

    /// How many pairs of numbers to add at the end
    #[arg(short, long, default_value_t = 1)]
    numbers: u32,

    /// How many symbols to add at the end
    #[arg(short = 'S', long, default_value_t = 0)]
    symbols: u32,
}

fn main() {
    let args = Args::parse();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v', 'x', 'z',
    ];
    let symbols = ['!', '@', '#', '%', '^', '&', '/', '=', '+'];

    let mut password = String::new();
    {
        let mut letter_type = args.start;
        let mut rng = rand::thread_rng();

        for _ in 0..7 {
            password.push(match letter_type {
                LetterType::Consonant => consonants[rng.gen_range(0..consonants.len())],
                LetterType::Vowel => vowels[rng.gen_range(0..vowels.len())],
            });

            letter_type = letter_type.other();
        }
        if letter_type == args.end {
            password.push({
                match letter_type {
                    LetterType::Consonant => consonants[rng.gen_range(0..consonants.len())],
                    LetterType::Vowel => vowels[rng.gen_range(0..vowels.len())],
                }
            });
        }

        for _ in 0..args.numbers {
            let number = rng.gen_range(0..9);
            password.push_str(number.to_string().as_str());
            password.push_str((number + 1).to_string().as_str());
        }

        for _ in 0..args.symbols {
            password.push(symbols[rng.gen_range(0..symbols.len())]);
        }
    }

    println!("{}", password);
}
