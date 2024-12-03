# Rustic Encode

This is a simple Rustic Encode program that encodes an iso-8859-1
file content to utf-8.

## Installation

```sh
git clone git@github.com:TheLionCoder/rustic-encode.git
cd rustic-encode
mkdir -p data/raw data/processed
```

> [!NOTE]
> data/raw is where the iso-8850-1 files are stored

## Usage

To run the program, use the following comands:

```bash
cargo build --release
```

_then:_

```bash
target/release/rustic-encode
```
