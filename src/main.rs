#![feature(seek_stream_len)]
use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
    thread,
};

use argh::FromArgs;
use bus::Bus;

#[derive(FromArgs, Debug)]
/// Simple interface to write image to many files/devices at once, can also be used to backup to multiple locations
struct Diode {
    #[argh(option, short = 'i')]
    /// input file to read from
    input: PathBuf,

    #[argh(option, short = 'o')]
    /// output file(s) to write to
    output: Vec<PathBuf>,

    #[argh(option, short = 'b', default = "64000")]
    /// set the block size to process data (default: 64000)
    block_size: usize,

    #[argh(option, short = 'm', default = "20")]
    /// set the amount of blocks to store in memory at a given time
    block_buffer: usize,

    #[argh(option, short = 'c')]
    /// number of blocks to read, useful for generating random data
    block_count: Option<usize>,
}

impl Diode {
    fn run(self) -> Result<Status, Box<std::io::Error>> {
        let mut message_bus: Bus<Vec<u8>> = Bus::new(self.block_buffer);
        let input_path = self.input.clone();
        let outputs = self.output.clone();

        let writer_threads: Vec<thread::JoinHandle<Result<(), std::io::Error>>> = outputs
            .into_iter()
            .map(|output_path| {
                let mut recv = message_bus.add_rx();
                thread::spawn(move || {
                    let mut file = File::create(&output_path)?;

                    loop {
                        match recv.recv() {
                            Ok(bytes) => {
                                file.write_all(&bytes)?;
                            }
                            Err(_err) => {
                                file.sync_all()?;
                                break;
                            }
                        }
                    }

                    Ok(())
                })
            })
            .collect();

        let reader_thread: thread::JoinHandle<Result<usize, std::io::Error>> =
            thread::spawn(move || {
                let mut file = File::open(input_path)?;

                let mut read = 0;

                match self.block_count {
                    Some(count) => {
                        let mut counter = 0;
                        while counter < count {
                            let mut tmp_buf = vec![0; self.block_size];
                            read += file.read(&mut tmp_buf)?;
                            message_bus.broadcast(tmp_buf);

                            counter += 1;
                        }
                    }
                    None => {
                        while file.stream_position()? < file.stream_len()? {
                            let diff = (file.stream_len()? - file.stream_position()?) as usize;

                            let mut tmp_buf = if diff < self.block_size {
                                vec![0; diff]
                            } else {
                                vec![0; self.block_size]
                            };
                            read += file.read(&mut tmp_buf)?;
                            message_bus.broadcast(tmp_buf);
                        }
                    }
                };

                Ok(read)
            });

        // Wait on threads
        let bytes_read = reader_thread.join().unwrap()?;

        let st = Status {
            bytes_copied: bytes_read,
            num_of_files: writer_threads.len(),
        };

        for handle in writer_threads {
            handle.join().unwrap().unwrap();
        }

        Ok(st)
    }
}

struct Status {
    bytes_copied: usize,
    num_of_files: usize,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} bytes copied to {} files.",
            self.bytes_copied, self.num_of_files
        )
    }
}

fn main() {
    let args: Diode = argh::from_env();
    let runner = args.run().unwrap();
    println!("{runner}");
}
