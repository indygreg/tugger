// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;

pub fn evaluate_glob(cwd: &str, pattern: &str) -> Vec<PathBuf> {
    let search = if pattern.starts_with('/') {
        pattern.to_string()
    } else {
        format!("{}/{}", cwd, pattern)
    };

    let mut res = Vec::new();

    for path in glob::glob(&search).unwrap() {
        let path = path.unwrap();

        if path.is_file() {
            res.push(path);
        }
    }

    res
}
