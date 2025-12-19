use clap::{Parser, Subcommand, ValueEnum};
use fract::Fract;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;
use std::time::Instant;

const BANNER: &str = r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║    ░██████╗░█████╗░░█████╗░██╗░░██╗░█████╗░████████╗░█████╗░  ║
    ║    ██╔════╝██╔══██╗██╔══██╗██║░░██║██╔══██╗╚══██╔══╝██╔══██╗  ║
    ║    █████╗░░███████║██║░░╚═╝███████║███████║░░░██║░░░██║░░╚═╝  ║
    ║    ██╔══╝░░██╔══██║██║░░██╗██╔══██║██╔══██║░░░██║░░░██║░░██╗  ║
    ║    ██║░░░░░██║░░██║╚█████╔╝██║░░██║██║░░██║░░░██║░░░╚█████╔╝  ║
    ║    ╚═╝░░░░░╚═╝░░╚═╝░╚════╝░╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░░╚════╝░  ║
    ║                                                              ║
    ║    Hyperchaotic · Quantum-Resistant · Fast Cryptographic Hash ║
    ║                                                              ║
    ║    Author: @morphym                                          ║
    ║    Version: 0.1.0                                            ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝
"#;

/// Fract - A hyperchaotic cryptographic hash function
///
/// Calculates Fract-256/512 hashes for files or standard input,
/// similar to sha256sum or md5sum.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// File(s) to hash (defaults to stdin if not provided)
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Use 512-bit output mode (enhanced quantum resistance)
    #[arg(short = '5', long = "512")]
    use_512: bool,

    /// Check hash values against a list (reads hashes from file)
    #[arg(short = 'c', long = "check")]
    check: bool,

    /// Verbose output mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Use binary mode output (backwards compatibility)
    #[arg(short = 'b', long = "binary")]
    binary: bool,

    /// Warn about improperly formatted checksum lines
    #[arg(short = 'w', long = "warn")]
    warn: bool,

    /// Hash algorithm variant (future-proofing)
    #[arg(value_enum, short = 'a', long = "algorithm", default_value = "fract")]
    algorithm: Algorithm,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run built-in benchmarks
    Bench {
        /// Test data size in bytes
        #[arg(short = 's', long = "size", default_value = "1048576")]
        size: usize,

        /// Number of iterations
        #[arg(short = 'i', long = "iter", default_value = "100")]
        iterations: usize,

        /// Test 512-bit mode
        #[arg(short = '5', long = "512")]
        use_512: bool,

        /// Test incremental hashing
        #[arg(short = 'c', long = "chunked")]
        chunked: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum Algorithm {
    Fract,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Fract => write!(f, "FRACT"),
        }
    }
}

fn main() {
    let args = Args::parse();

    // Show banner if no arguments provided
    if args.files.is_empty() && args.command.is_none() && !args.check {
        println!("{}", BANNER);
        println!("Usage: fract [OPTIONS] [FILE]...");
        println!("       fract bench [OPTIONS]");
        println!();
        println!("Run 'fract --help' for detailed usage information.");
        std::process::exit(0);
    }

    if let Some(command) = args.command {
        match command {
            Commands::Bench {
                size,
                iterations,
                use_512,
                chunked,
            } => {
                run_benchmark(size, iterations, use_512, chunked);
            }
        }
    } else if args.check {
        if args.files.is_empty() {
            eprintln!("Error: --check requires at least one file argument");
            std::process::exit(1);
        }
        for check_file in &args.files {
            if let Err(e) = check_hashes(check_file, &args) {
                eprintln!("Error checking {}: {}", check_file.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        let result = if args.files.is_empty() {
            hash_stdin(&args)
        } else {
            hash_files(&args.files, &args)
        };

        if let Err(e) = result {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_benchmark(size: usize, iterations: usize, use_512: bool, chunked: bool) {
    println!("=== Fract Benchmark ===");
    println!("Data size: {} bytes", size);
    println!("Iterations: {}", iterations);
    println!("Mode: {}", if use_512 { "512-bit" } else { "256-bit" });
    println!(
        "Method: {}",
        if chunked { "chunked" } else { "single-pass" }
    );
    println!();

    // Create test data
    let data = vec![0x61u8; size]; // 'a' repeated

    // Warmup
    for _ in 0..10 {
        if use_512 {
            let _ = Fract::hash512(&data);
        } else {
            let _ = Fract::hash(&data);
        }
    }

    // Benchmark
    let start = Instant::now();
    let mut hash = [0u8; 32];

    if chunked {
        let chunk_size = 4096.min(size);
        for _ in 0..iterations {
            let mut hasher = Fract::new();
            for chunk in data.chunks(chunk_size) {
                hasher.update(chunk);
            }
            if use_512 {
                hash.copy_from_slice(&hasher.finalize());
            } else {
                hash = hasher.finalize();
            }
        }
    } else {
        for _ in 0..iterations {
            if use_512 {
                let hash512 = Fract::hash512(&data);
                hash.copy_from_slice(&hash512[..32]);
            } else {
                hash = Fract::hash(&data);
            }
        }
    }

    let elapsed = start.elapsed();
    let total_bytes = (size * iterations) as f64;
    let throughput = total_bytes / elapsed.as_secs_f64();
    let throughput_mib = throughput / (1024.0 * 1024.0);

    println!("Total time: {:?}", elapsed);
    println!("Throughput: {:.2} MiB/s", throughput_mib);
    println!("Last hash: {}", hex::encode(&hash[..8]));
    println!();

    // Additional stats
    println!("=== Additional Stats ===");
    println!("Bytes processed: {}", total_bytes as usize);
    println!(
        "Nanoseconds per byte: {:.2}",
        (elapsed.as_nanos() as f64) / total_bytes
    );
    if !use_512 {
        println!(
            "Cycles/byte (est. at 3GHz): {:.2}",
            (elapsed.as_nanos() as f64 * 3.0) / total_bytes
        );
    }
}

fn hash_stdin(args: &Args) -> io::Result<()> {
    let mut buffer = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer)?;

    let hash = if args.use_512 {
        hex::encode(Fract::hash512(&buffer))
    } else {
        hex::encode(Fract::hash(&buffer))
    };

    if args.binary {
        println!("{} *-{}", hash, "-");
    } else {
        println!("{}  -", hash);
    }

    Ok(())
}

fn hash_files(files: &[PathBuf], args: &Args) -> io::Result<()> {
    for file_path in files {
        if file_path.to_string_lossy() == "-" {
            hash_stdin(args)?;
            continue;
        }

        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("fract: {}: {}", file_path.display(), e);
                continue;
            }
        };

        let mut buffer = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            eprintln!("fract: {}: {}", file_path.display(), e);
            continue;
        }

        let hash = if args.use_512 {
            hex::encode(Fract::hash512(&buffer))
        } else {
            hex::encode(Fract::hash(&buffer))
        };

        if args.verbose {
            let mode_char = if args.binary { '*' } else { ' ' };
            println!(
                "{} {}{}  {}{}",
                args.algorithm.to_string().to_uppercase(),
                hash,
                mode_char,
                if args.use_512 { "512" } else { "256" },
                file_path.display()
            );
        } else if args.binary {
            println!("{} *{}", hash, file_path.display());
        } else {
            println!("{}  {}", hash, file_path.display());
        }
    }

    Ok(())
}

fn check_hashes(check_file: &PathBuf, args: &Args) -> io::Result<()> {
    let file = File::open(check_file)?;
    let reader = BufReader::new(file);
    let mut failed = false;
    let mut line_num = 0;

    for line in reader.lines() {
        line_num += 1;
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse hash line: <hash> <mode><filename>
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            if args.warn {
                eprintln!(
                    "{}:{}: improperly formatted line",
                    check_file.display(),
                    line_num
                );
            }
            continue;
        }

        let expected_hash = parts[0];
        let file_spec = parts[1];

        // Extract filename (handle * or space prefix)
        let filename = if file_spec.starts_with('*') {
            &file_spec[1..]
        } else {
            file_spec.trim_start()
        };

        let file_path = PathBuf::from(filename);
        if !file_path.exists() && filename != "-" {
            eprintln!("{}: FAILED open or read", filename);
            failed = true;
            continue;
        }

        let mut file = File::open(&file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let actual_hash = if expected_hash.len() == 128 {
            hex::encode(Fract::hash512(&buffer))
        } else {
            hex::encode(Fract::hash(&buffer))
        };

        if actual_hash == expected_hash {
            if args.verbose {
                println!("{}: OK", filename);
            }
        } else {
            eprintln!("{}: FAILED", filename);
            if args.verbose {
                eprintln!("  Expected: {}", expected_hash);
                eprintln!("  Actual:   {}", actual_hash);
            }
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data for consistency";
        let hash1 = hex::encode(Fract::hash(data));
        let hash2 = hex::encode(Fract::hash(data));
        assert_eq!(hash1, hash2);

        let hash512_1 = hex::encode(Fract::hash512(data));
        let hash512_2 = hex::encode(Fract::hash512(data));
        assert_eq!(hash512_1, hash512_2);
    }
}
