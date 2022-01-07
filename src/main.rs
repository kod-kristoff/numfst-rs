use std::{fs, io};
use clap::{AppSettings, Parser};
use log::LevelFilter;

use numfst::{IntSet, IntSetBuilder};


type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let args = Args::parse();
    if let Err(err) = try_main(args) {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

#[derive(clap::Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// init the prime transducer
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Init {
        /// the file to write to
        file: String,
        /// the limit
        limit: usize,
    },
    /// query the prime transducer
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Check {
        /// the file to read the transducer from
        file: String,
        /// the number to test if it is prime
        num: u32,
    },
}
fn try_main(args: Args) -> Result<()> {
//    let log_level = if args.quiet() {
//        LevelFilter::Off
//    } else if args.debug() {
//        LevelFilter::Debug
//    } else if args.verbose() {
//        LevelFilter::Info
//    } else {
//        LevelFilter::Warn
//    };
    let log_level = LevelFilter::Trace;
    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    log::debug!("args = {:?}", args);
    // println!("Using file '{}' as input", args.input());
    match args.command {
        Commands::Init{ file, limit } => {
            log::trace!("init file={}, limit={}", file, limit);

            let primes = sieve(limit);
            log::trace!("primes = {:?}", primes);
            log::info!("# primes: {}, size: {} bytes", primes.len(), primes.len() * 4);
            let wtr = io::BufWriter::new(fs::File::create(file)?);
            let mut builder = IntSetBuilder::new(wtr)?;

            for prime in primes {
                builder.insert(prime)?;
            }
            log::info!("written: {} bytes", builder.bytes_written());
            builder.finish()?;
        },
        Commands::Check{ file, num } => {
            log::trace!("check if {} is prime", num);
            let primes = IntSet::new(fs::read(file)?)?;

            match primes.contains(num) {
                true => println!("{} is a prime", num),
                false => println!("{} is not a prime", num),
            };
        }
    };
    Ok(())
}

fn sieve(n: usize) -> Vec<u32> {
    log::trace!("generating primes up to {}", n);
    if n <= 1 {
        return vec![];
    }

    let mut marked = vec![true; n+1];
    marked[0] = false;
    marked[1] = false;

    for i in 2..((n as f64).sqrt().ceil() as usize) {
        if marked[i] {
            let mut j = i * i;
            let mut k = 0;
            while j <= n {
                marked[j] = false;
                k += 1;
                j = i*i + k*i;
            }
        }
    }
    marked.iter()
        .take(n)
        .enumerate()
        .filter_map(|(i, &m)| if !m { None } else { Some(i as u32) })
        .collect()
}
