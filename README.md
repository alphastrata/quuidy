# quuidy

CLI application to generate v4 type UUIDs, `-n` number of them.

## Usage
To generate v4 UUIDs, use the following command:

```bash
./quuidy -n [number]
```

Replace [number] with the desired number of UUIDs to generate.

## Prerequisites
Make sure you have Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).

## Install
```shell
cargo install quuidy
```

## Build

To build the application, run:

```bash
cargo build --release
```

## Run

To run the application, use the following command:

```bash
./target/release/uuid_generator -n [number]
# or, if installed via cargo
quuidy -n [number]
```

