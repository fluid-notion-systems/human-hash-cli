use clap::Parser;
use human_hash::humanize;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "human-hash")]
#[command(about = "Generate human-readable identifiers using human-hash-rs", long_about = None)]
struct Cli {
    /// Number of words to use (1-8)
    #[arg(short, long, default_value_t = 4)]
    words: usize,

    /// Use a custom separator between words
    #[arg(short, long, default_value = "-")]
    separator: String,
}

fn main() {
    let cli = Cli::parse();

    // Validate words parameter
    if cli.words == 0 || cli.words > 8 {
        eprintln!("Error: words must be between 1 and 8");
        std::process::exit(1);
    }

    let uuid =
        // Generate random UUID
        Uuid::new_v4();

    // Generate human-readable hash using the library
    let hash = humanize(&uuid, cli.words);

    // Apply custom separator if needed
    let output = if cli.separator == "-" {
        hash
    } else {
        hash.replace("-", &cli.separator)
    };

    println!("{}", output);
}
