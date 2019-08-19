// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use slog::warn;
use starlark::environment::Environment;
use starlark::values::{default_compare, TypedValue, Value, ValueError, ValueResult};
use starlark::{any, immutable, not_supported};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

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

impl TarArchive {
    pub fn execute(&self, logger: &slog::Logger, dist_path: &Path) -> Result<(), String> {
        let dest_path = dist_path.join(&self.dest_name);

        warn!(logger, "writing tarball to {}", dest_path.display());

        std::fs::create_dir_all(dest_path.parent().unwrap()).or_else(|_| {
            Err(format!(
                "unable to create directory for {}",
                dest_path.display()
            ))
        })?;

        let fh = std::fs::File::create(&dest_path).or_else(|_| {
            Err(format!(
                "unable to open {} for writing",
                dest_path.display()
            ))
        })?;

        let mut builder = tar::Builder::new(fh);
        builder.mode(tar::HeaderMode::Deterministic);

        for (rel_path, fs_path) in &self.file_manifest.files {
            warn!(logger, "adding {} as {}", fs_path.display(), rel_path);
            builder
                .append_path_with_name(fs_path, rel_path)
                .or_else(|e| Err(e.to_string()))?;
        }

        builder.finish().or_else(|e| Err(e.to_string()))?;

        Ok(())
    }
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

/// Represents a generic step.
#[derive(Debug, Clone)]
pub enum Step {
    Snap(super::snap::Snap),
    TarArchive(TarArchive),
}

/// Represents a series of `Step`s to execute.
#[derive(Debug, Default, Clone)]
pub struct Pipeline {
    /// The name of this pipeline.
    pub name: String,

    /// Path to write distribution files.
    pub dist_path: PathBuf,

    /// The series of steps to execute.
    pub steps: Vec<Step>,
}

impl TypedValue for Pipeline {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("Pipeline<name={}, steps={:#?}", self.name, self.steps)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "Pipeline"
    }

    fn to_bool(&self) -> bool {
        false
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}
