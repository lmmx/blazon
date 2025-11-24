use facet::Facet;

#[derive(Facet)]
pub struct Args {
    /// Path to README file to update
    #[facet(positional, default = "README.md".to_string())]
    pub readme: String,

    /// Crate name for crates.io link (default: infer from Cargo.toml)
    #[facet(named, short = 'c', long, default)]
    pub crate_name: Option<String>,

    /// Binary name to measure (default: infer from Cargo.toml)
    #[facet(named, short = 'b', long, default)]
    pub binary: Option<String>,

    /// Skip building in release mode before measuring
    #[facet(named, long, default)]
    pub no_build: bool,

    /// Show verbose output
    #[facet(named, short = 'v', long, default)]
    pub verbose: bool,

    /// Show this help message
    #[facet(named, short = 'h', long, default)]
    pub help: bool,
}

pub fn print_usage() {
    println!("Usage: blazon [OPTIONS] [README]");
    println!();
    println!("Generate and update badge metadata in README files.");
    println!();
    println!("Arguments:");
    println!("  [README]             Path to README file (default: README.md)");
    println!();
    println!("Options:");
    println!("  -c, --crate-name NAME  Crate name for badges (default: infer)");
    println!("  -b, --binary NAME      Binary name to measure (default: infer)");
    println!("  --no-build             Skip building");
    println!("  -v, --verbose          Show verbose output");
    println!("  -h, --help             Show this help message");
}
