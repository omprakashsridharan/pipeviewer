use pipeviewer::{args, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
    let args = args::Args::parse();
    let mut total_bytes = 0;
    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes);
        if write::write(&args.outfile, &buffer)? {
            break;
        }
    }
    Ok(())
}
