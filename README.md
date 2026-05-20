# grep-clone

A minimal yet performant clone of `grep` written in Rust. Recursively search files for text patterns with colored output, parallel processing, and more.

## Features

- **Pattern Matching** – Find lines containing a query string (case-sensitive by default).
- **Case-Insensitive Search** (`-i`) – Match regardless of letter case.
- **Recursive Directory Search** (`-r`) – Walk through directories and search all regular files.
- **Line Numbers** (`-n`) – Print line numbers alongside matches.
- **Colored Output** – Query matches are highlighted in green, file paths in magenta/bold.
- **Binary File Detection** – Binary files (containing a null byte) are automatically skipped.
- **Parallel Processing** – When `-r` is used, files are processed in parallel with `rayon` for speed.
- **Standalone Integration Test** – Contains a test that runs the binary end-to-end.

## Usage

```
grep-clone [OPTIONS] <QUERY> <FILE_PATH>
```

### Arguments

| Argument      | Description                          |
|---------------|--------------------------------------|
| `QUERY`       | The text pattern to search for       |
| `FILE_PATH`   | Path to a file or directory (with -r)|

### Options

| Flag           | Description                              |
|----------------|------------------------------------------|
| `-i`           | Perform a case-insensitive search        |
| `-r`           | Recursively search directories           |
| `-n`           | Show line numbers in output              |
| `-h` / `--help`| Print help information                   |
| `-V` / `--version`| Print version information            |

### Examples

Search a single file:

```
cargo run -- "hello" myfile.txt
```

Case-insensitive recursive search with line numbers:

```
cargo run -- -i -r -n "rust" ./src
```

Search a directory recursively:

```
cargo run -- -r "fn main" ./src
```

## Installation

### From source

```bash
git clone https://github.com/den319/grep-clone.git
cd grep-clone
cargo build --release
./target/release/grep-clone --help
```

### Run without installing

```bash
cargo run -- <QUERY> <FILE_PATH> [OPTIONS]
```

## Project Structure

```
grep-clone/
├── Cargo.toml          # Dependencies & metadata
├── src/
│   ├── main.rs         # Entry point, CLI orchestration
│   ├── config.rs       # CLI argument struct (clap derive)
│   └── search.rs       # Search logic & binary detection
└── tests/
    └── grep_test.rs    # Integration tests
```

### Modules

- **`config`** – Uses `clap` to define and parse command-line arguments (`query`, `file_path`, `ignore_case`, `recursive`, `line_numbers`).
- **`search`** – Provides:
  - `search()` – Case-sensitive line-by-line match.
  - `search_case_insensitive()` – Case-insensitive variant.
  - `is_binary()` – Heuristic to detect binary files (looks for a null byte).
  - Unit tests for the search functions.
- **`main`** – Orchestrates the search: decides single-file vs. recursive mode, skips binary files, parallelises with `rayon` under `-r`, and prints results with colored highlighting.

## Dependencies

| Crate      | Purpose                              |
|------------|--------------------------------------|
| `clap`     | CLI argument parsing (derive API)    |
| `colored`  | Terminal output coloring             |
| `rayon`    | Parallel iteration for file scanning |
| `walkdir`  | Recursive directory traversal        |

## Testing

```bash
# Run unit tests (search functions)
cargo test

# Run integration tests (spawns the binary)
cargo test --test grep_test
```

The integration test suite:
- Verifies basic file search output.
- Stress-tests parallel recursive search on 200 files without crashing.
