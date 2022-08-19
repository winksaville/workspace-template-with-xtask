# Workspace template with xtask's

A workspace with xtasks. This is based on [cargo-xtask](https://github.com/matklad/cargo-xtask)
and [rusty-ferris-club/rust_starter](https://github.com/rusty-ferris-club/rust-starter).

To use this as a template simply `git clone https://github.com/winksaville/workspace-template-with-xtask new-repo-dirctory`
this repo, remove the `.git` directory and then modify as you see fit.
See [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

## Building

The folloing sections define tasks, "scripts" written in rust,
which maybe executed with either `cargo xtask xxx` or `cargo xt xxx`.
Where `xxx` is one of the `tasks` below:

> See [cargo/config](.cargo/config)

### clippy

Runs `cargo clippy` in the current directory

### fmt

Runs `cargo fmt` in the current directory

### test

Runs `cargo test` in the current directory

### pre-commit

Runs `cargo fmt`, `cargo clipppy` and `cargo test` from the project root

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

