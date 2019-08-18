// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use starlark::environment::Environment;
use starlark::values::{default_compare, TypedValue, Value, ValueError, ValueResult};
use starlark::{any, immutable, not_supported};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
}

impl TypedValue for SourceFile {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("SourceFile<path={:?}>", self.path)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "SourceFile"
    }

    fn to_bool(&self) -> bool {
        false
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

/// Represents a virtual filesystem mapping of relative filenames to source files.
///
/// This may expand in the future to abstract the source of file content so
/// that file data can come from memory, etc. We may also want to add per-file
/// metadata, such as the owner, permissions, etc. For now things are simple.
#[derive(Debug, Default, Clone)]
pub struct FileManifest {
    pub files: BTreeMap<String, PathBuf>,
}

impl TypedValue for FileManifest {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("FileManifest<{:#?}", self.files)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "FileManifest"
    }

    fn to_bool(&self) -> bool {
        false
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

/// Represents a step to produce a tar archive.
#[derive(Debug, Clone)]
pub struct TarArchive {
    /// Filename of produced tar archive.
    pub dest_name: String,

    /// Manifest denoting content to be added to archive.
    pub file_manifest: FileManifest,
}

impl TypedValue for TarArchive {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!(
            "TarArchive<dest_name={}, file_manifest={:#?}",
            self.dest_name, self.file_manifest
        )
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "TarArchive"
    }

    fn to_bool(&self) -> bool {
        false
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}
