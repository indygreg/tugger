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

See the `tugger.ship` file in the repository for examples of how Tugger
uses itself to self-distribute.

## Status

Tugger is still very alpha and rough around the edges. There is a lot
still planned, especially around ergonomics of the Starlark configuration
files (we want to drastically reduce the amount of boilerplate required
to perform common tasks, for example).
