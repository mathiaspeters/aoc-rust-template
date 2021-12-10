# aoc-rust-template

This project is an empty template for doing AdventOfCode in Rust. 

The template is structured so that each day is it's own module with a source file, an input file and a test input file. In a test context the test input will be read, otherwise the normal input will be read. There are already unit tests that call the associated processing functions, just update the expected answer for each assert.

Finally, you can run using one of the scripts. For powershell run `./run.ps1 -Day <the day you want to run>` and for bash run `/run.sh <the day you want to run>`. In the current format all, the code for all days will be run but only the results for the specified day will be shown.

Since powershell doesn't have grep out of the box, you need to install ripgrep before you can run. Ripgrep can be installed with cargo: `cargo install ripgrep`.
