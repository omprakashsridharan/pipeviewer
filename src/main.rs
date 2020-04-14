use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Result, Write};
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new("Pipeviewer")
        .arg(Arg::with_name("infile").help("Read a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s"))
        .get_matches();
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        if !silent {
            eprintln!("Bytes read: {}", total_bytes);
        }
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break Ok(()),
            Ok(x) => x,
            Err(_) => break Ok(()),
        };
        total_bytes += num_read;
        writer.write_all(&buffer[..num_read])?
    }
}
