# Advent of Code

This is my solutions for the [Advent of Code](https://adventofcode.com/) challenges.

[![Rust](https://img.shields.io/badge/language-rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Just](https://img.shields.io/badge/command_runner-just-skyblue?style=for-the-badge)](https://just.systems/)

[![Cargo Workspace](https://img.shields.io/badge/organized_with-cargo_workspace-blue)](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
[![Cargo Watch](https://img.shields.io/badge/developed_with-cargo_watch-blue)](https://watchexec.github.io/#cargo-watch)
[![Cargo Generate](https://img.shields.io/badge/code_generation-cargo_generate-blue)](https://cargo-generate.github.io/cargo-generate/)

## Table of Contents

- [Usage](#usage)
  - [Prerequisites](#prerequisites)
  - [Generate a new day](#generate-a-new-day)
  - [Working on one part of a day](#working-on-one-part-of-a-day)
  - [Running tests](#running-tests)
  - [Running lint](#running-lint)
  - [Running a part with the input](#running-a-part-with-the-input)
- [References](#references)

## Usage

### Prerequisites

```shell
cargo install cargo-generate cargo-watch just
```

> **Note**: You can also use `cargo binstall` instead of `cargo install` if you prefer.

### Generate a new day

```shell
just create <day>
```

### Working on one part of a day

```shell
just work <day> <part>
```

### Running tests

```shell
just test <day>
```

### Running lint

```shell
just lint <day>
```

### Running a part with the input

```shell
just run <day> <part>
```

> **Note**: The input is expected to be placed in the `src/input.txt` file.

## References

- Project setup: [How to set up Rust for Advent of Code](https://www.youtube.com/watch?v=fEQv-cqzbPg)
