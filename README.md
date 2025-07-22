# human-hash-cli

A command-line tool to generate human-readable, memorable identifiers using the [human-hash-rs](https://github.com/jamesmunns/human-hash-rs) library.

## Installation

```bash
cd human-hash-cli
cargo build --release
```

The binary will be available at `target/release/human-hash-cli`.

## Usage

```bash
# Generate a random human-readable hash (default: 4 words)
./human-hash-cli
# Example output: autumn-alaska-piano-three

# Generate with different number of words (1-8)
./human-hash-cli --words 3
# Example output: cup-sweet-december

# Hash a specific input string (deterministic)
./human-hash-cli --input "hello world"
# Example output: east-lima-spaghetti-may

# Use a specific UUID
./human-hash-cli --uuid "7528880a-986c-40e7-8c38-115e640da2a1"
# Example output: three-georgia-xray-jig

# Use custom separator
./human-hash-cli --separator "_" --words 2
# Example output: cup_sweet

# Generate multiple random identifiers
./human-hash-cli --count 5
# Example output:
# pasta-magazine-social-yankee
# sink-salami-charlie-aspen
# golf-neptune-jupiter-network
# six-oxygen-early-early
# harry-colorado-failed-alabama
```

## Options

- `-w, --words <WORDS>`: Number of words to use (1-8) [default: 4]
- `-s, --separator <SEPARATOR>`: Custom separator between words [default: -]
- `-c, --count <COUNT>`: Generate multiple identifiers [default: 1]
- `-i, --input <INPUT>`: Hash input string instead of generating random UUIDs
- `-u, --uuid <UUID>`: Use specific UUID instead of generating random ones
- `-h, --help`: Print help information

## Features

### Random Generation
By default, generates random human-readable identifiers using UUID v4:

```bash
# Different each time
./human-hash-cli --count 3
```

### Deterministic Hashing
When you provide an input string, the same string will always produce the same human-readable hash:

```bash
./human-hash-cli --input "my-project-v1.0"
# Always outputs the same hash for this input
```

### UUID-based Hashing
You can provide a specific UUID to get its human-readable representation:

```bash
./human-hash-cli --uuid "550e8400-e29b-41d4-a716-446655440000" --words 4
```

### Custom Separators
Change the separator between words for different use cases:

```bash
# For file names
./human-hash-cli --separator "_"

# For domains
./human-hash-cli --separator "."

# For paths
./human-hash-cli --separator "/"
```

### Variable Word Count
Adjust the number of words (1-8) to balance between memorability and uniqueness:

```bash
# Short identifier (higher collision probability)
./human-hash-cli --words 2

# Long identifier (very low collision probability)
./human-hash-cli --words 8
```

## Use Cases

```bash
# Generate unique run identifiers
RUN_ID=$(./human-hash-cli)
mkdir "runs/$RUN_ID"

# Create reproducible project codenames from git commits
./human-hash-cli --input "$(git rev-parse HEAD)" --words 3

# Generate test data with consistent naming
./human-hash-cli --input "test-case-1" --separator "_"

# Create memorable temporary file names
TEMP_FILE="/tmp/$(./human-hash-cli --words 2).txt"

# Generate human-friendly API keys or tokens
./human-hash-cli --words 6 --separator "."
```

## About human-hash-rs

This CLI uses the [human-hash-rs](https://github.com/jamesmunns/human-hash-rs) library by James Munns, which is inspired by the Python [humanhash](https://github.com/zacharyvoase/humanhash) library. The library provides:

- A curated list of 256 words for optimal hash distribution
- Deterministic output for the same UUID input
- Support for variable-length outputs (1-8 words)
- Collision resistance appropriate to the number of words used

The word list includes simple, memorable English words that are easy to pronounce and spell, making the hashes suitable for human communication and memory.