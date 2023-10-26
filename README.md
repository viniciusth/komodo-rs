# Komodo - Simplifying competitive programming for the Rust language
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/viniciusth/komodo-rs/blob/master/LICENSE)

Tired of having to copy-paste your library code into every solution you write? Komodo is a command-line tool that allows you to easily create, expand, run and test Rust solutions for competitive programming problems.

This project is more of a template than an actual library. It provides useful CLI tools for competitive programming, but you will have to write your own code and library for the problems you want to solve.

There are some example solutions and libraries in this repository, but they are mostly for reference and testing purposes, so you should probably delete them and write your own.

## Installation
Make sure you have Rust installed. Then, clone this repository (or simply create your own repository from this template):
```bash
git clone https://github.com/viniciusth/komodo-rs.git
```
And then you should be able to run the CLI with:
```bash
cd komodo-rs
cargo run
```

## Usage
```bash
Komodo CLI for Rust Competitive Programming

Usage: cargo run <COMMAND>

Commands:
  expand  Expands a solution file into a submission file, saves to clipboard by default
  code    Runs main of src/code/{question}.rs
  stress  Stress tests a solution with a brute force solution
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Solution files and library
The project starts out with 3 solution files, but it can be easily extended by adding more questions to the `questions!` macro call in `src/code/mod.rs` and creating the corresponding `.rs` file following the same template as `src/code/template.rs`.

You can use Rust normally in your solution and library files, but all imports must be from either the standard library or the crate so that the code can be correctly expanded by the `cargo run expand` command.

#### Expanding
The `cargo run expand` command will expand a solution file into a submission file. By default, it will save the submission file to the clipboard, but you can also save it to a file with the `-o` flag.

#### Running
The `cargo run code` command will run the `main` function of the solution file corresponding to the question you pass as an argument. For example, `cargo run code -q b` will run the `main` function of `src/code/b.rs`.

#### Stress testing
The `cargo run stress [-q]` command will stress test a solution with a brute force solution. It will run both solutions on random inputs and compare the results. If they are different, it will print the input and the diff of the outputs. The brute force solution and generator files are both located in `src/code/stress/`.
