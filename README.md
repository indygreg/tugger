# Tugger

Tugger is a generic application packaging and distribution tool.

Tugger provides its core functionality as a Rust crate (`tugger`). This
functionality is exposed through the `tugger` command line tool.

Tugger is typically configured via `tugger.ship` files. These files
use a custom [Starlark](https://github.com/bazelbuild/starlark) dialect
for defining packaging and distribution actions.

Most of the documentation for Tugger lives as part of the Rust crate.
Run `cargo doc --open` to build the documentation and open it in a web
browser.
