// Import the standard library's I/O module so we can read from stdin.
// use std::io;
use std::{
    // env,
    // error::Error,
    io,
    // ffi::OsString,
    fs::File,
    // process,
};
use clap::{Parser, ArgAction};
use csv::Reader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag(true))]
struct Args {
    #[arg(short = '?', action = ArgAction::Help)]
    help: bool,
    
    #[arg(short, long, value_parser = clap::value_parser!(String))]
    file: String,

    #[arg(short, long, default_value = ",")]
    delimiter: String,
    
    // if true output the record count
    #[arg(short, long, default_value_t = false)]
    count: bool,
}

// The `main` function is where your program starts executing.
fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let mut rdr: Reader<Box<dyn io::Read>>;
    if args.file == "" || args.file == "-" {
        rdr = csv::ReaderBuilder::new()
            .delimiter(args.delimiter.as_bytes()[0])
            .flexible(true)
            .from_reader(Box::new(io::stdin()));
    } else {
        rdr = csv::ReaderBuilder::new()
            .delimiter(args.delimiter.as_bytes()[0])
            .flexible(true)
            .from_reader(Box::new(match File::open(&args.file) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error opening file '{}': {}", args.file, e);
                std::process::exit(1);
            }
        }));
    }
    if args.count {
        let count = rdr.records().count();
        println!("{:?} records", count);
        return;     
    }
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
}
