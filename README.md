# aoc-rust-template

This project is an empty template for doing AdventOfCode in Rust. 

The template is structured so that each day is it's own module with a source file, an input file and a test input file. In a test context the test input will be read, otherwise the normal input will be read. There are already unit tests that call the associated processing functions, just update the expected answer for each assert.

When running you can pass the days you want to run like `cargo run -- 1` or `cargo run -- 5 10 15` and only those days will be run.
