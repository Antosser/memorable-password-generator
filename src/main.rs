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

    /// Whether to add a number at the end
    #[arg(short, long, default_value_t = true)]
    number: bool,
}

fn main() {
    let args = Args::parse();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 's', 't', 'v', 'x', 'z',
    ];

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
            password.push(match letter_type {
                LetterType::Consonant => consonants[rng.gen_range(0..consonants.len())],
                LetterType::Vowel => vowels[rng.gen_range(0..vowels.len())],
            });
        }

        if args.number {
            let number = rng.gen_range(0..9);
            password.push_str(number.to_string().as_str());
            password.push_str((number + 1).to_string().as_str());
        }
    }

    println!("{}", password);
}
