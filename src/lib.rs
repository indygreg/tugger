// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/*!
The `tugger` crate contains functionality for packaging and distributing
software applications.

The core of `tugger` consists of a set of types for defining packaging
actions and functions to operate on them. There is a frontend component
which defines a [Starlark](https://github.com/bazelbuild/starlark)
dialect for allowing these types to be constructed from user-provided
configuration files. A command-line interface is also provided to
perform common actions.

## Data Model

At its core, Tugger is a pipeline execution engine where the primitives
composing pipelines are tasks related to packaging and distribution.
Tugger can be thought of a domain-specific build system.

Tugger executes primitives call `pipelines`. Each `pipeline` is an
entity with a name, some attached execution context (e.g. the working
directory), and an ordered series of `steps` to execute. Each `step`
is simply some action that can be executed. Actions include things
like *produce a tarball* or *run snapcraft*.

## Starlark Config Files

Tugger uses Starlark to allow defining `pipelines` in rich configuration
files. These configuration files can be evaluated and their results
turned into `pipeline` execution actions via the `tugger` command-line
tool or by calling appropriate functions in this crate.

Tugger configuration files are named `tugger.ship` by convention.

See the [`starlark`](starlark/index.html) module for documentation of the
Starlark dialect.
*/

pub mod cli;
pub mod debian;
pub mod filemanifest;
pub mod glob;
pub mod snap;
#[allow(unused)]
pub mod starlark;
