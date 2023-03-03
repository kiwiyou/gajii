# Gajii

`Gajii` is an interpreter for [`Gaji` language.](https://purring-advantage-060.notion.site/d4990596b21340f2a4e1e5bcc649f4d0)

## Installation

Install [Rust](https://www.rust-lang.org/) and run the following commands:

```bash
git clone https://github.com/kiwiyou/gajii.git
cd gajii
cargo install .
```

After installation, check if `$PATH` contains `$HOME/.cargo/bin`.

## Usage

```
Usage: gajii [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Name of the gaji source to run

Options:
  -e, --echo     If set, print the code to stderr before run
  -h, --help     Print help
  -V, --version  Print version
```
