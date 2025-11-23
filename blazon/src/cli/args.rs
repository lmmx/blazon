use facet::Facet;

#[derive(Facet)]
pub struct Args {
    /// Path(s) to source file(s) or directory to format
    #[facet(positional, default = vec![".".to_string()])]
    pub sources: Vec<String>,

    /// Check mode: exit with error if files need formatting
    #[facet(named, short = 'c', long, default)]
    pub check: bool,

    /// Write formatted output back to files (default: print to stdout)
    #[facet(named, short = 'w', long, default)]
    pub write: bool,

    /// Show verbose output
    #[facet(named, short = 'v', long, default)]
    pub verbose: bool,

    /// Show this help message
    #[facet(named, short = 'h', long, default)]
    pub help: bool,
}

pub fn print_usage() {
    println!("Usage: blazon [OPTIONS] [SOURCES]...");
    println!();
    println!("A minimal Rust code formatter.");
    println!();
    println!("Arguments:");
    println!("  [SOURCES]...       Path(s) to file(s) or directory to format (default: 'src')");
    println!();
    println!("Options:");
    println!("  -c, --check        Check if files need formatting (exit 1 if so)");
    println!("  -w, --write        Write formatted output back to files");
    println!("  -v, --verbose      Show verbose output");
    println!("  -h, --help         Show this help message");
    println!();
    println!("Examples:");
    println!("  # Format a single file and print to stdout");
    println!("  blazon src/lib.rs");
    println!();
    println!("  # Format multiple files");
    println!("  blazon src/*.rs");
    println!();
    println!("  # Check if files need formatting");
    println!("  blazon --check src/");
    println!();
    println!("  # Format files in-place");
    println!("  blazon --write src/");
}
