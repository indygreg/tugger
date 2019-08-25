// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod cli;
pub mod debian;
pub mod filemanifest;
pub mod glob;
pub mod snap;
pub mod starlark;

fn main() {
    if let Err(e) = cli::run_cli() {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}
