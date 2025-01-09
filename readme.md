# Tetris in Rust

A simple terminal-based Tetris game implemented in Rust. The game is played directly in the terminal and provides a nostalgic block-dropping experience with a modern Rust twist.

## Features

- **Terminal-based gameplay**: Fully interactive Tetris game in your terminal.
- **Rust-powered performance**: Built using the Rust programming language.
- **Code quality tools**: Integrated support for formatting, linting, and testing via `cargo`.

## Prerequisites

- **Rust and Cargo**: Install via [rustup](https://rustup.rs/).
- **Make**: Required to use the provided `Makefile`.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/roman91DE/tetris.git
   cd tetris
   ```

2. Check tools and dependencies:
   ```bash
   make check-tools
   ```

3. Format and lint the code:
   ```bash
   make format
   make lint
   ```

## Usage

### Build and Run

- **Debug mode**:
  ```bash
  make build
  make run
  ```

- **Release mode**:
  ```bash
  make release
  make run-release
  ```

### Testing

Run tests to ensure everything is working:
```bash
make test
```

### Cleaning

Clean up build artifacts:
```bash
make clean
```

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve the game.

## License

This project is licensed under the [MIT License](LICENSE).
