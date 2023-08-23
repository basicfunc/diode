#![feature(seek_stream_len)]
use std::{
    eprintln,
    fs::File,
    io::{Read, Seek, Write},
    path::PathBuf,
    thread,
};

use argh::FromArgs;
use bus::Bus;
use thiserror::Error;

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
    fn run(self) -> Result<Status, Error> {
        let mut message_bus: Bus<Vec<u8>> = Bus::new(self.block_buffer);
        let input_path = self.input.clone();
        let outputs = self.output.clone();

        let writer_threads: Vec<thread::JoinHandle<Result<(), Error>>> = outputs
            .into_iter()
            .map(|output_path| {
                let mut recv = message_bus.add_rx();
                thread::spawn(move || {
                    let mut file = File::create(&output_path)
                        .map_err(|_| Error::UnableToCreateFile(output_path))?;

                    loop {
                        match recv.recv() {
                            Ok(bytes) => {
                                file.write_all(&bytes)
                                    .map_err(|_| Error::UnableToWriteToBuffer)?;
                            }
                            Err(_) => {
                                file.sync_all().map_err(|_| Error::UnableToSyncFiles)?;
                                break;
                            }
                        }
                    }

                    Ok(())
                })
            })
            .collect();

        let reader_thread: thread::JoinHandle<Result<usize, Error>> = thread::spawn(move || {
            let mut file =
                File::open(&input_path).map_err(|_| Error::UnableToOpenFile(input_path.clone()))?;

            let mut read = 0;

            match self.block_count {
                Some(count) => {
                    let mut counter = 0;
                    while counter < count {
                        let mut tmp_buf = vec![0; self.block_size];
                        read += file
                            .read(&mut tmp_buf)
                            .map_err(|_| Error::UnableToReadBytesFrom(input_path.clone()))?;
                        message_bus.broadcast(tmp_buf);

                        counter += 1;
                    }
                }
                None => {
                    let input = input_path.clone();
                    loop {
                        let curr_pos = file
                            .stream_position()
                            .map_err(|_| Error::UnableToGetCurrPos(input.clone()))?;

                        let full_len = file
                            .stream_len()
                            .map_err(|_| Error::UnableToGetByteLen(input.clone()))?;

                        if curr_pos < full_len {
                            let diff = (full_len - curr_pos) as usize;

                            let mut tmp_buf = if diff < self.block_size {
                                vec![0; diff]
                            } else {
                                vec![0; self.block_size]
                            };

                            read += file
                                .read(&mut tmp_buf)
                                .map_err(|_| Error::UnableToReadBytesFrom(input.clone()))?;
                            message_bus.broadcast(tmp_buf);
                        } else {
                            break;
                        }
                    }
                }
            };

            Ok(read)
        });

        // Wait on threads
        let bytes_read = reader_thread
            .join()
            .map_err(|_| Error::FailedToJoinThreads)??;

        let st = Status {
            bytes_copied: bytes_read,
            num_of_files: writer_threads.len(),
        };

        for handle in writer_threads {
            handle.join().map_err(|_| Error::FailedToJoinThreads)??;
        }

        Ok(st)
    }
}

struct Status {
    bytes_copied: usize,
    num_of_files: usize,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Error ocuured while writing to buffer.")]
    UnableToWriteToBuffer,
    #[error("Error ocuured while syncing all files.")]
    UnableToSyncFiles,
    #[error("Error ocuured while creating file: {0}.")]
    UnableToCreateFile(PathBuf),
    #[error("Error ocuured while opening file: {0}.")]
    UnableToOpenFile(PathBuf),
    #[error("Error ocuured while reading bytes from file: {0}.")]
    UnableToReadBytesFrom(PathBuf),
    #[error("Error ocuured while getting current seek position of file: {0}.")]
    UnableToGetCurrPos(PathBuf),
    #[error("Error ocuured while getting last seek position of file: {0}.")]
    UnableToGetByteLen(PathBuf),
    #[error("Error ocuured while waiting for threads.")]
    FailedToJoinThreads,
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
    match argh::from_env::<Diode>().run() {
        Ok(st) => println!("{st}"),
        Err(err) => eprintln!("{err}"),
    }
}
