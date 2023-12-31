use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(short, long)]
    pub print_config: bool,
}

impl Cli {
    pub fn init() -> Self {
        Cli::parse()
    }
}
