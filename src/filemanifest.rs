// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Install files in a files manifest to a destination directory.
pub fn install_files(dest_dir: &Path, files: &BTreeMap<String, PathBuf>) {
    for (key, source_path) in files.iter() {
        let rel_path = PathBuf::from(key);
        let rel_dir = rel_path.parent().unwrap();

        let d = dest_dir.join(rel_dir);
        if !d.exists() {
            std::fs::create_dir_all(&d).expect("unable to create directory");
        }

        let dest_path = dest_dir.join(rel_path);
        std::fs::copy(&source_path, &dest_path).expect("unable to copy file");
    }
}
