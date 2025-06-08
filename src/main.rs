use anyhow::{Context, Result};
use std::{
    error::Error,
    io,  // so we can read from stdin.
    fs::File,
    process,
};
use std::io::Read;
use clap::Parser;
use csv::Reader;

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None, disable_help_flag(true))]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = clap::value_parser!(String))]
    file: String,

    #[arg(short, long, default_value = ",", 
        help = "The delimiter to use when parsing the CSV")]
    delimiter: String,
    
    // if true output the record count
    #[arg(short, long, default_value_t = false, 
        help = "If this option is given, cq outputs the count of records in the CSV and exits")]
    count: bool,

    #[arg(short, long, default_value_t = false, 
        help = "If this option is given, cq outputs the (h)eader of the CSV and exits")]
    eader: bool,

    #[arg(short, long, default_value_t = false,
        help = "This option tells cq that the CSV file has no header. It and the (h)eader option don't get along")]
    noheader: bool,

    #[arg(short, long, default_value_t = -1)]
    index: i16 // a maximum positive value of 32767 ought to be enough for anybody :-)
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let delimiter = args.delimiter.replace("\\t", "\t");
    let has_header = !args.noheader;
    let mut rdr: Reader<Box<dyn io::Read>>;
    // Read from stdin
    if args.file == "" || args.file == "-" {
        rdr = get_stdin_reader(delimiter, has_header);
    } else {
        let file = File::open(&args.file).with_context(|| format!("Failed to open file {}", &args.file))?;
        rdr = get_file_reader(delimiter, has_header, file);
    }
    if args.count {
        println!("{:?} records", rdr.records().count());
        return Ok(());
    }
    if args.eader && !args.noheader {
        println!("{:?}", rdr.headers().context("Failed to get headers")?);
        return Ok(());
    }

    if args.index > -1 {
        let index = args.index as usize;
        // Loop over each record.
        for result in rdr.records() {
            let record = result.context("Failed to get CSV record")?;
            println!("{}", record.get(index)
                .with_context(|| format!("Failed to get CSV record at index {}", index))?
                .trim_end());
        }
    } else {
        for result in rdr.records() {
            let record = result.context("Failed to get CSV record")?;
            println!("{:?}", record);
        }
    }
    Ok(())
}

fn get_file_reader(delimiter: String, has_header:bool, file: File) -> Reader<Box<dyn Read>> {
    get_reader_from_input(delimiter, has_header, Box::new(file))
}

fn get_stdin_reader(delimiter: String, has_header: bool) -> Reader<Box<dyn Read>> {
    get_reader_from_input(delimiter, has_header, Box::new(io::stdin()))
}
pub fn get_reader_from_input(delimiter: String, has_header: bool, input: Box<dyn Read>) -> Reader<Box<dyn Read>> {
    csv::ReaderBuilder::new()
        .delimiter(delimiter.as_bytes()[0])
        .has_headers(has_header)
        .flexible(true)
        .from_reader(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_file_reader_reads_csv_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "name,age\nAlice,30\nBob,25").unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let delimiter = ",".to_string();
        let reader = get_file_reader(delimiter, true, file);

        let records: Vec<_> = reader.into_records().map(|r| r.unwrap()).collect();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get(0).unwrap(), "Alice");
        assert_eq!(records[1].get(1).unwrap(), "25");
    }

    #[test]
    fn test_get_file_reader_reads_csv_data_without_headers() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Alice,30\nBob,25").unwrap();

        let file = File::open(temp_file.path()).unwrap();
        let delimiter = ",".to_string();
        let reader = get_file_reader(delimiter, false, file);

        let records: Vec<_> = reader.into_records().map(|r| r.unwrap()).collect();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get(0).unwrap(), "Alice");
        assert_eq!(records[1].get(1).unwrap(), "25");
    }


    #[test]
    fn test_get_reader_from_input_reads_tab_delimited_data() {
        let input_data = "name\tage\nCharlie\t40\nDana\t35";
        let input = Cursor::new(input_data);

        let delimiter = "\t".to_string();
        let reader = get_reader_from_input(delimiter, true, Box::new(input));
        // let reader = get_stdin_reader(delimiter); // This hangs because it waits for stdin

        let records: Vec<_> = reader.into_records().map(|r| r.unwrap()).collect();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].get(0).unwrap(), "Charlie");
        assert_eq!(records[1].get(1).unwrap(), "35");
    }
}