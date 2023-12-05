# AdventOfCode

This is my solutions for the [Advent of Code](https://adventofcode.com/) challenges.

## Table of Contents

- [Technologies](#technologies)
- [Usage](#usage)
  - [Prerequisites](#prerequisites)
  - [Generate a new day](#generate-a-new-day)
  - [Working on one part of a day](#working-on-one-part-of-a-day)
  - [Running tests](#running-tests)
  - [Running lint](#running-lint)
- [References](#references)

## Technologies

- [Rust](https://www.rust-lang.org/)
- [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Cargo Generate](https://cargo-generate.github.io/cargo-generate/)
- [Just](https://just.systems/man/en/)

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

## References

- Project setup: [How to set up Rust for Advent of Code](https://www.youtube.com/watch?v=fEQv-cqzbPg)
