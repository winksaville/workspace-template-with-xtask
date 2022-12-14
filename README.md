# Workspace template with xtask's

A workspace with xtasks. This is based on [cargo-xtask](https://github.com/matklad/cargo-xtask)
and [rusty-ferris-club/rust_starter](https://github.com/rusty-ferris-club/rust-starter).

To use this as a template simply `git clone https://github.com/winksaville/workspace-template-with-xtask new-repo-dirctory`
this repo, remove the `.git` directory and then modify as you see fit.
See [workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html).

## Using

The following sections define tasks, "scripts" written in rust,
which maybe executed with either `cargo xtask xxx` or `cargo xt xxx`.
Where `xxx` is one of the `Tasks` below:

> See [cargo/config](.cargo/config) for the aliases

Tasks
 * pre-commit
   * Runs `cargo fmt`, `cargo clippy` and `cargo test` in \<proj-root\>

 * gen-phl
   * Removes <proj-root>/coverage/ then generates coverage data in <proj-root>/coverage/
   using gen-profraw, gen-html and gen-lcov.
   [Click to see coverage/html](https://htmlpreview.github.io/?https://github.com/winksaville/workspace-template-with-xtask/blob/main/coverage/html/index.html)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

