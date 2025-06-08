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
use std::io::Read;
use clap::{Parser, ArgAction};
use csv::Reader;

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None, disable_help_flag(true))]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short = '?', action = ArgAction::Help)]
    // usage: bool,
    
    #[arg(short, long, value_parser = clap::value_parser!(String))]
    file: String,

    #[arg(short, long, default_value = ",")]
    delimiter: String,
    
    // if true output the record count
    #[arg(short, long, default_value_t = false)]
    count: bool,

    #[arg(short, long, default_value_t = false)]
    eader: bool,
    
    #[arg(short, long, default_value_t = -1)]
    index: i32
}


fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let mut rdr: Reader<Box<dyn io::Read>>;
    // Read from stdin
    if args.file == "" || args.file == "-" {
        rdr = get_stdin_reader(&args);
    } else {
        // or read from a file
        let file = match File::open(&args.file) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error opening file '{}': {}", args.file, e);
                std::process::exit(1);
            }
        };
        rdr = get_file_reader(&args, file);
    }
    if args.count {
        let count = rdr.records().count();
        println!("{:?} records", count);
        return;     
    }
    
    if args.index > -1 {
        let index = args.index as usize;
        // Loop over each record.
        for result in rdr.records() {
            // An error may occur, so abort the program in an unfriendly way.
            // We will make this more friendly later!
            let record = result.expect("a CSV record");
            println!("{}", match record.get(index) {
                Some(x) => x,
                None => todo!(),
            }.trim_end());
        }
    } else { 
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            println!("{:?}", record);
        }
    }
}

fn get_file_reader(args: &Args, file: File) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(args.delimiter.as_bytes()[0])
        .flexible(true)
        .from_reader(Box::new(file))
}

fn get_stdin_reader(args: &Args) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(args.delimiter.as_bytes()[0])
        .flexible(true)
        .from_reader(Box::new(io::stdin()))
}
