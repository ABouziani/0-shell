# o-shell

o-shell is a simple Unix-like shell implemented in Rust. It supports basic shell commands such as `cd`, `pwd`, and more, providing a lightweight and educational shell environment.

## Features

- Change directory (`cd`)
- Print working directory (`pwd`)
- Execute system commands
- Basic error handling

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Building

Clone the repository and build the project:

```bash
git clone https://github.com/ABouziani/0-shell.git
cd o-shell
cargo build --release
```

### Running

To start the shell:

```bash
cargo run
```

## Usage

Once running, you can use commands like:

- `cd [directory]` — Change the current directory
- `pwd` — Print the current working directory
- Other system commands

## Project Structure

- `src/commands/` — Contains command implementations (e.g., `cd.rs`, `pwd.rs`)
- `src/main.rs` — Entry point for the shell

