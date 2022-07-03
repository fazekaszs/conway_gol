# conway_gol

## About

A simple Rust pogram that "plays" Conway's Game of Life in the terminal. This is one of my first Rust projects, 
so the implementation is probably far off from optimal.

## Prerequisites

This software does not have any dependencies.

## How to Compile

After downloading the code, install the Rust compiler: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`.
This command works on Linux and MacOS. For more detailed instructions click [here](https://doc.rust-lang.org/book/ch01-01-installation.html).
If you want to check whether cargo is in your PATH run `which cargo`. In case it is not sourced yet, run `source ~/.cargo/env`. After
establishing that cargo is in your PATH go to the project directory and run

```bash
cargo build --release
```

This will procude a project_home/target/release directory, in which you can find the conway_gol executable.

## How to Use

Running the binary prompts the user to provide an initial cell configuration. Currently, the acceptable answers to this are "glider",
"oscillator", "glidergun", and "custom". The first three configurations are hardcoded into the source code. If the user chooses the
"custom" option, the next prompt will ask for a path to an initial configuration file. This path must be an absolute path! The initial
configuration file must only contain `#` characters and spaces, indicating alive and dead cells, respectively. The file can have variable
length lines. After choosing the configuration the CLI will periodically update, showing the current state of the system. Keep in mind, that
if the system becomes bigger than the terminal, only the bottom left corner of the system will be displayed.
