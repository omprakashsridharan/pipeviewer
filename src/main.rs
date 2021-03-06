use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || {
        read::read_loop(&infile, stats_tx);
    });
    let stats_handle = thread::spawn(move || {
        stats::stats_loop(silent, stats_rx, write_tx);
    });
    let write_handle = thread::spawn(move || {
        write::write_loop(&outfile, write_rx);
    });

    let read_io_result = read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    // let a = read_io_result?;
    // let b = write_io_result?;
    // let c = stats_io_result?;
    Ok(())
}
