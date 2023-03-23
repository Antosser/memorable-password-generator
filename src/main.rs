use clap::Parser;
use rand::Rng;

/// Create memorizable passwords easily using this CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether to add a random symbol at the end
    #[arg(short, long, default_value_t = false)]
    symbol: bool,

    /// The amount of words
    #[arg(short, long, default_value_t = 2)]
    words: u32,

    /// The amount of number pairs
    #[arg(short, long, default_value_t = 1)]
    numberspairs: u32,
}

fn main() {
    let args = Args::parse();
    let nounlist: Vec<&str> = std::include_str!("nounlist.txt").split('\n').collect();

    let mut password = "".to_owned();
    let mut rng = rand::thread_rng();

    for _ in 0..args.words {
        password.push_str(nounlist[rng.gen_range(0..nounlist.len())]);
    }

    for _ in 0..args.numberspairs {
        let num = rng.gen_range(0..=8);

        password.push_str(num.to_string().as_str());
        password.push_str((num + 1).to_string().as_str());
    }

    if args.symbol {
        let symbols = "!@#$%^&*-+=".to_owned();

        password.push(
            symbols
                .chars()
                .nth(rng.gen_range(0..symbols.len()))
                .unwrap(),
        );
    }

    println!("{}", password);
}
