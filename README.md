# Diode: Multi-Writer ISO Burning Tool

Diode is a command-line tool designed to simplify the process of burning ISO images to multiple files or devices simultaneously. It provides an interface to efficiently copy data from an input ISO file to multiple output files or devices in parallel. Additionally, it offers options to control block sizes, buffer sizes, and more to optimize the copying process.

## Features

- **Parallel Writing**: Diode employs multiple writer threads to copy data from the input ISO file to multiple output files or devices concurrently. This speeds up the copying process, especially when dealing with large ISO images or multiple destinations.

- **Customizable Block Size**: Users can specify the block size, which determines the amount of data copied in each iteration. This allows for fine-tuning performance and resource usage based on the specific use case.

- **Memory Buffering**: Diode allows users to set the number of blocks to store in memory at a given time. This provides control over memory usage and can help prevent excessive memory consumption during the copying process.

- **Random Data Generation**: The tool offers the option to generate random data by specifying the number of blocks to read. This feature can be useful for testing or generating randomized ISO files.

## Installation

Currently, Diode is available as a standalone command-line tool. To use Diode, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/basicfunc/diode
   ```

2. Build the tool using the Rust programming language's package manager, Cargo:

   ```bash
   cd diode
   cargo build --release
   ```

3. The compiled binary will be available in the `target/release` directory. You can move it to a location in your system's PATH for easy access.

## Usage

Diode can be run from the command line with various options. Below is an example usage:

```bash
diode -i input.iso -o output1.img output2.img -b 8192 -m 10 -c 100
```

- `-i`: Specifies the input ISO file to read data from.
- `-o`: Specifies the output file(s) or device(s) to write data to.
- `-b`: Sets the block size for processing data (default: 64000 bytes).
- `-m`: Sets the number of blocks to store in memory at a given time.
- `-c`: Specifies the number of blocks to read (useful for generating random data).

## GUI Interface (Upcoming)

An upcoming enhancement to Diode is the addition of a graphical user interface (GUI) developed using Flutter. This GUI will provide a more user-friendly way to interact with the tool, making it easier to burn ISO images to USB drives and other devices. The GUI will offer a streamlined interface for configuring settings, selecting input and output files/devices, and monitoring the copying progress.

Stay tuned for updates on the GUI version of Diode!

## Contributions

Diode is an open-source project, and contributions are welcome. If you'd like to contribute to the project, feel free to fork the repository, make your changes, and submit a pull request.

## License

Diode is distributed under the [MIT License](LICENSE). You are free to use, modify, and distribute this software in accordance with the terms of the license.