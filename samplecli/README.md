# Surta copy of the book `introduction to rust programming` chapter 4

A rust command-line application using [`clap`](https://github.com/clap-rs/clap) crate.

This contains some binaries, main bin (`./src/main.rs`) and other bins (`.src/bin/err_anyhow.rs`, `.src/bin/err_panic.rs`, and `.src/bin/err_thiserror.rs`) 

So to run the bin, you need to specify a name.

e.g.
```shell
$ cargo run --bin rpncalc
```
or
```shell
$ cargo run --bin err_anyhow
```
