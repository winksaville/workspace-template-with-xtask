# Tasks for automation

Based on https://github.com/winksaville/workspace-template-with-xtask, which
is based on https://github.com/matklad/cargo-xtask.

Tasks
 * pre-commit
   * Runs `cargo fmt`, `cargo clippy` and `cargo test` in \<proj-root\>

 * gen-phl
   * Removes <proj-root>/coverage/ then generates coverage data in <proj-root>/coverage/
   using gen-profraw, gen-html and gen-lcov.
   [Click to see coverage/html](https://htmlpreview.github.io/?https://github.com/winksaville/workspace-template-with-xtask/blob/main/coverage/html/index.html)
