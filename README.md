# nizer

**nizer** is a terminal-based file navigator and text file opener written in Rust. It allows you to browse directories, view metadata, and open text files using the `micro` editor, all from your terminal.

## Features

- Navigate directories interactively from the terminal.
- View and select files and folders.
- Open text files (`.md`, `.txt`, `.doc`, `.html`) directly in the [micro](https://micro-editor.github.io/) editor.
- Displays file metadata (modification and creation dates).

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- [micro](https://micro-editor.github.io/) editor installed and available in your `$PATH`

### Build

Clone the repository and build with Cargo:

```sh
git clone https://github.com/yourusername/nizer.git
cd nizer
cargo build --release
```

The binary will be in `target/release/nizer`.

## Usage

Run `nizer` in your terminal:

```sh
./target/release/nizer [directory]
```

- If no directory is provided, it starts in the current directory.
- Use the interactive prompt to navigate and open files.
- Press `q` to quit.

## Dependencies

- [walkdir](https://crates.io/crates/walkdir)
- [chrono](https://crates.io/crates/chrono)
- [crossterm](https://crates.io/crates/crossterm)
- [ratatui](https://crates.io/crates/ratatui)

All dependencies are listed in [`Cargo.toml`](Cargo.toml).

## Contributing

Contributions are welcome! Please open issues or pull requests.

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.

## License

This project is licensed under the MIT License. See [`LICENSE`](LICENSE) for details.

## Author

- [Pablo Romero](https://github.com/90PabloRomero)
