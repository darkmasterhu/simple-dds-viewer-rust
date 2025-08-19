# DDS Viewer

A simple windowed application written in Rust to view `.dds` (DirectDraw Surface) texture files.

This project uses:
- [`ddsfile`](https://crates.io/crates/ddsfile) for low-level DDS file parsing.
- [`image_dds`](https://crates.io/crates/image_dds) for decoding DDS images into a viewable format.
- [`minifb`](https://crates.io/crates/minifb) for displaying the decoded image in a native window.

## Features

- Open and display `.dds` images in a window.
- Supports common DDS formats via `image_dds`.
- Simple and lightweight — minimal dependencies.

## Installation

Make sure you have Rust and Cargo installed. If not, install them via [rustup.rs](https://rustup.rs).

Clone the repository:

```bash
git clone https://github.com/darkmasterhu/simple-dds-viewer-rust.git
cd dds_viewer
```

## Usage

To build and run the application:

```bash
cargo run -- path_to_file.dds
```

Replace `path_to_file.dds` with the path to your DDS image.

### Example:

```bash
cargo run -- assets/example.dds
```

The image will open in a new window.

## Requirements

- Rust 1.76+ (edition 2024)
- A supported platform for [`minifb`](https://github.com/emoon/rust_minifb) (Windows, macOS, Linux)

## Project Structure

- `main.rs` — Loads a DDS file, decodes it, and displays it.
- `Cargo.toml` — Project metadata and dependencies.

## Dependencies

```toml
[dependencies]
ddsfile = "0.5.2"
minifb = "0.28.0"
image_dds = "0.7.2"
```
