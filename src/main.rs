// Import the standard library's I/O module so we can read from stdin.
// use std::io;
use std::io::Write;
use std::{
    // env,
    // error::Error,
    io,
    // ffi::OsString,
    fs::File,
    // process,
};
use std::io::Read;
use clap::Parser;
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
    index: i16 // a max value of 32767
}


fn main() {
    let args = Args::parse();
    //println!("{:?}", args);
    let delimiter = args.delimiter.replace("\\t", "\t");
    let mut rdr: Reader<Box<dyn io::Read>>;
    // Read from stdin
    if args.file == "" || args.file == "-" {
        rdr = get_stdin_reader(delimiter);
    } else {
        // or read from a file
        let file = match File::open(&args.file) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error opening file '{}': {}", args.file, e);
                std::process::exit(1);
            }
        };
        rdr = get_file_reader(delimiter, file);
    }
    if args.count {
        let count = rdr.records().count();
        println!("{:?} records", count);
        return;     
    }
    if args.eader {
        match rdr.headers() {
            Ok(headers) => {
                println!("{:?}", headers);
                return;
            }
            Err(e) => {
                eprintln!("Error reading headers: {}", e);
                std::process::exit(1);
            }
        };
    }

    if args.index > -1 {
        let index = args.index as usize;
        // Loop over each record.
        for result in rdr.records() {
            // An error may occur, so abort the program in an unfriendly way.
            // We will make this more friendly later!
            let record = result.expect("a CSV record");
            println!("{}", record.get(index).unwrap_or("Failed to get Index").trim_end());
        }
    } else {
        for result in rdr.records() {
            let record = result.expect("a CSV record");
            println!("{:?}", record);
        }
    }
}

fn get_file_reader(delimiter: String, file: File) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .flexible(true)
        .from_reader(Box::new(file))
}

fn get_stdin_reader(delimiter: String) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .flexible(true)
        .from_reader(Box::new(io::stdin()))
}
pub fn get_reader_from_input(delimiter: String, input: Box<dyn Read>) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .flexible(true)
        .from_reader(input)
}

#[test]
fn test_get_file_reader_reads_csv_data() {
    use tempfile::NamedTempFile;
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "name,age\nAlice,30\nBob,25").unwrap();

    let file = File::open(temp_file.path()).unwrap();
    let delimiter = ",".to_string();
    let reader = get_file_reader(delimiter, file);

    let records: Vec<_> = reader.into_records().map(|r| r.unwrap()).collect();

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].get(0).unwrap(), "Alice");
    assert_eq!(records[1].get(1).unwrap(), "25");
}

#[test]
fn test_get_reader_from_input_reads_tab_delimited_data() {
    use std::io::Cursor;
    let input_data = "name\tage\nCharlie\t40\nDana\t35";
    let input = Cursor::new(input_data);

    let delimiter = "\t".to_string();
    let reader = get_reader_from_input(delimiter, Box::new(input));

    let records: Vec<_> = reader.into_records().map(|r| r.unwrap()).collect();

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].get(0).unwrap(), "Charlie");
    assert_eq!(records[1].get(1).unwrap(), "35");
}