#[cfg(feature = "cli")]
pub mod cli {
    pub mod args;
    pub mod report;

    use args::{Args, print_usage};
    use blazon_core::{collect_metrics, generate_badges, get_binary_name, update_readme};
    use std::io;

    pub fn main() -> io::Result<()> {
        // Install miette handler for nice error displays
        if let Err(e) = report::install_handler() {
            eprintln!("Warning: Failed to install error handler: {}", e);
        }

        let args: Args = facet_args::from_std_args().map_err(|e| {
            eprintln!("{}", report::DiagnosticDisplay(&e));
            io::Error::new(io::ErrorKind::InvalidInput, "Argument parsing failed")
        })?;

        if args.help {
            print_usage();
            std::process::exit(0);
        }

        let binary_name = match &args.binary {
            Some(name) => name.clone(),
            None => get_binary_name().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                eprintln!("Please specify --binary NAME");
                std::process::exit(1);
            }),
        };

        if args.verbose {
            eprintln!("Binary: {}", binary_name);
            eprintln!("README: {}", args.readme);
            eprintln!();
        }

        let metrics = collect_metrics(&binary_name, !args.no_build).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

        if args.verbose {
            eprintln!("Dependencies: {}", metrics.dep_count);
            eprintln!("Binary size: {} bytes", metrics.binary_size_bytes);
            eprintln!();
        }

        let crate_name = match &args.crate_name {
            Some(name) => name.clone(),
            None => binary_name.clone(), // Use binary name as fallback
        };

        let badges = generate_badges(&metrics, &crate_name);

        if args.verbose {
            eprintln!("Generated badges:");
            eprintln!("{}", badges);
            eprintln!();
        }

        eprintln!("Updating {}...", args.readme);
        update_readme(&args.readme, &badges).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

        eprintln!("✓ Successfully updated {}", args.readme);
        Ok(())
    }
}

#[cfg(not(feature = "cli"))]
pub mod cli {
    pub fn main() {
        eprintln!("Please build with the cli feature");
        eprintln!("Example: cargo install blazon --features cli");
        std::process::exit(1);
    }
}

pub use cli::main;
