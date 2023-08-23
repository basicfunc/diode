use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Simple interface to write image to many files/devices at once, can also be used to backup to multiple locations
struct Args {
    #[argh(option, short = 'i')]
    /// input file to read from (if left empty STDIN is used)
    input: Option<PathBuf>,

    #[argh(option, short = 'o')]
    /// output file(s) to write to. (if left empty STDOUT is used)
    output: Option<PathBuf>,

    #[argh(option, short = 'b', default = "64000")]
    /// set the block size to process data (default: 64000)
    block_size: usize,

    #[argh(option, short = 'm', default = "20")]
    /// set the amount of blocks to store in memory at a given time
    block_buffer: usize,

    #[argh(option, short = 'c')]
    /// number of blocks to read, useful for generating random data from /dev/random or zeroing drives
    block_count: Option<usize>,
}

fn main() {
    let args: Args = argh::from_env();
    println!("{args:?}");
}
