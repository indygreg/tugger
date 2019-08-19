// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use starlark::environment::{Environment, EnvironmentError};
use starlark::stdlib::global_functions;
use starlark::values::list::List;
use starlark::values::{
    RuntimeError, Value, ValueError, ValueResult, INCORRECT_PARAMETER_TYPE_ERROR_CODE,
};
use starlark::{
    check_type, starlark_err, starlark_fun, starlark_module, starlark_signature,
    starlark_signature_extraction, starlark_signatures,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub mod eval;
mod snap;
mod values;

use values::{FileManifest, Pipeline, SourceFile, Step, TarArchive};

fn evaluate_glob(cwd: &str, pattern: &str) -> Vec<PathBuf> {
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

fn resolve_include_exclude(cwd: &str, include: &Value, exclude: &Value) -> ValueResult {
    let mut result = HashSet::new();

    // Evaluate all the includes first.
    match include.get_type() {
        "string" => {
            for p in evaluate_glob(&cwd, &include.to_str()) {
                result.insert(p);
            }
        }
        "list" => {
            for v in include.into_iter()? {
                if v.get_type() != "string" {
                    return Err(ValueError::TypeNotX {
                        object_type: v.get_type().to_string(),
                        op: "string".to_string(),
                    });
                }

                for p in evaluate_glob(&cwd, &v.to_str()) {
                    result.insert(p);
                }
            }
        }
        t => {
            return Err(ValueError::TypeNotX {
                object_type: t.to_string(),
                op: "string".to_string(),
            });
        }
    }

    // Then apply excludes.
    match exclude.get_type() {
        "NoneType" => {}
        "string" => {
            for p in evaluate_glob(&cwd, &exclude.to_str()) {
                result.remove(&p);
            }
        }
        "list" => {
            for v in exclude.into_iter()? {
                if v.get_type() != "string" {
                    return Err(ValueError::TypeNotX {
                        object_type: v.get_type().to_string(),
                        op: "string".to_string(),
                    });
                }

                for p in evaluate_glob(&cwd, &v.to_str()) {
                    result.remove(&p);
                }
            }
        }
        t => {
            return Err(ValueError::TypeNotX {
                object_type: t.to_string(),
                op: "string".to_string(),
            });
        }
    }

    let paths_vec: Vec<Value> = result
        .iter()
        .map(|path| Value::new(SourceFile { path: path.clone() }))
        .collect();

    Ok(Value::new(List::from(paths_vec)))
}

fn optional_str_arg(name: &str, value: &Value) -> Result<Option<String>, ValueError> {
    match value.get_type() {
        "NoneType" => Ok(None),
        "string" => Ok(Some(value.to_str())),
        t => Err(RuntimeError {
            code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
            message: format!(
                "function expects an optional string for {}; got type {}",
                name, t
            ),
            label: format!("expected type string; got {}", t),
        }
        .into()),
    }
}

fn required_str_arg(name: &str, value: &Value) -> Result<String, ValueError> {
    match value.get_type() {
        "string" => Ok(value.to_str()),
        t => Err(RuntimeError {
            code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
            message: format!("function expects a string for {}; got type {}", name, t),
            label: format!("expected type string; got {}", t),
        }
        .into()),
    }
}

fn required_dict_arg(
    arg_name: &str,
    key_type: &str,
    value_type: &str,
    value: &Value,
) -> Result<(), ValueError> {
    match value.get_type() {
        "dict" => {
            for k in value.into_iter()? {
                match k.get_type() {
                    key_type => Ok(()),
                    t => Err(RuntimeError {
                        code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
                        message: format!(
                            "dict {} expects keys of type {}; got {}",
                            arg_name, key_type, t
                        ),
                        label: format!("expected type {}; got {}", key_type, t),
                    }
                    .into()),
                }?;

                let v = value.at(k.clone())?;

                match v.get_type() {
                    value_type => Ok(()),
                    t => Err(RuntimeError {
                        code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
                        message: format!(
                            "dict {} expects values of type {}; got {}",
                            arg_name, value_type, t,
                        ),
                        label: format!("expected type {}; got {}", value_type, t),
                    }
                    .into()),
                }?;
            }
            Ok(())
        }
        t => Err(RuntimeError {
            code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
            message: format!("function expects a dict for {}; got type {}", arg_name, t),
            label: format!("expected type dict; got {}", t),
        }
        .into()),
    }
}

starlark_module! { appdistribute_module =>
    /// glob(include, exclude=[])
    ///
    /// Resolve file patterns to files.
    ///
    /// `include` is a `str` or `list` of `str` containing filenames that will be
    /// matched using the `glob` Rust crate. If filenames begin with `/` they are
    /// absolute. Otherwise they are relative to the directory the file is
    /// being evaluated in.
    ///
    /// `exclude` has the same type as `include` but is used to exclude certain
    /// files from the result. All patterns in `include` are evaluated before
    /// `exclude`.
    ///
    /// Returns a `list` of `SourceFile` instances.
    glob(env env, include, exclude=None) {
        let cwd = env.get("CWD").unwrap().to_str();

        resolve_include_exclude(&cwd, &include, &exclude)
    }

    /// file_manifest_from_files(files, relative_to=None, prefix=None)
    ///
    /// Construct a `FileManifest` from an iterable of `SourceFile` (often from
    /// using `glob()`).
    ///
    /// The paths in `FileManifest` will be relative to the `relative_to` path,
    /// which is the relative directory the file is being evaluated in by default.
    ///
    /// `prefix` can be used to prefix all relative paths with a value.
    file_manifest_from_files(env env, files, relative_to=None, prefix=None) {
        let cwd = env.get("CWD").unwrap().to_str();

        if files.get_type() != "list" {
            return Err(ValueError::TypeNotX {
                object_type: files.get_type().to_string(),
                op: "list".to_string(),
            });
        }

        let relative_to_path = match relative_to.get_type() {
            "NoneType" => PathBuf::from(cwd),
            "string" => PathBuf::from(relative_to.to_str()),
            t => {
                return Err(ValueError::TypeNotX {
                    object_type: t.to_string(),
                    op: "str".to_string(),
                })
            },
        };

        let prefix = match prefix.get_type() {
            "NoneType" => None,
            "string" => Some(prefix.to_str()),
            t => {
                return Err(ValueError::TypeNotX {
                    object_type: t.to_string(),
                    op: "str".to_string(),
                })
            }
        };

        let mut manifest = FileManifest::default();

        for v in files.into_iter()? {
            if v.get_type() != "SourceFile" {
                return Err(ValueError::TypeNotX {
                    object_type: v.get_type().to_string(),
                    op: "SourceFile".to_string(),
                });
            }

            let raw_value = v.0.borrow();
            let source_file: &SourceFile = raw_value.as_any().downcast_ref().unwrap();
            let path = &source_file.path;
            let relative_path: &Path = path.strip_prefix(&relative_to_path)
                .or_else(|_| Err(ValueError::Runtime(RuntimeError {
                    code: "bad_relative_path",
                    message: format!("{} is not relative to {}", path.display(), relative_to_path.display()),
                    label: "relative_to".to_string(),
                })))?;

            let relative_path = match prefix {
                Some(ref prefix) => PathBuf::from(prefix).join(relative_path),
                None => relative_path.to_path_buf(),
            };

            manifest.files.insert(relative_path.display().to_string(), path.canonicalize().unwrap());
        }

        Ok(Value::new(manifest))
    }

    /// tar_archive(filename, manifest)
    ///
    /// Produce a tar archive from a manifest of files.
    ///
    /// `filename` is a string denoting the output filename.
    ///
    /// `manifest` is a `FileManifest` describing the files to add to the archive.
    /// The value will be copied and modifications to the original `FileManifest`
    /// will not be reflected on the returned instance.
    ///
    /// Returns a `TarArchive` describing a tar archive to produce.
    tar_archive(filename, manifest) {
        check_type!(filename, "tar_archive", string);
        check_type!(manifest, "tar_archive", FileManifest);

        let raw_manifest = manifest.0.borrow();
        let file_manifest: &FileManifest = raw_manifest.as_any().downcast_ref().unwrap();

        let tar = TarArchive {
            dest_name: filename.to_str(),
            file_manifest: file_manifest.clone(),
        };

        Ok(Value::new(tar))
    }

    /// pipeline(name, steps=[])
    ///
    /// Create a pipeline from a series of steps.
    ///
    /// Pipelines represent a named series of actions to perform to accomplish some
    /// task. A step is a type - like the result of `tar_archive()` - that can be
    /// evaluated.
    pipeline(env env, name, steps=None) {
        check_type!(name, "pipeline", string);

        let steps = if steps.get_type() == "NoneType" {
            List::new()
        } else {
            steps.clone()
        };

        check_type!(steps, "pipeline", list);

        let mut res = Vec::new();

        for step in steps.into_iter()? {
            let step = match step.get_type() {
                "TarArchive" => {
                    let raw_value = step.0.borrow();
                    let tar_archive: &TarArchive = raw_value.as_any().downcast_ref().unwrap();
                    Step::TarArchive(tar_archive.clone())
                },
                "Snap" => {
                    let raw_value = step.0.borrow();
                    let snap: &snap::Snap = raw_value.as_any().downcast_ref().unwrap();
                    Step::Snap(snap.clone())
                },
                t => {
                    return Err(ValueError::TypeNotX {
                        object_type: t.to_string(),
                        op: "pipeline".to_string(),
                    });
                },
            };

            res.push(step);
        }

        let dist_path: Value = env.get("DIST_PATH").unwrap();

        let pipeline = Value::new(Pipeline {
            name: name.to_str(),
            steps: res,
            dist_path: PathBuf::from(dist_path.to_str()),
        });

        let pipelines: Value = env.get("PIPELINES").unwrap();
        List::mutate(&pipelines, &|values: &mut Vec<Value>| {
            values.push(pipeline.clone());

            Ok(Value::from(None))
        })?;

        Ok(pipeline)
    }
}

/// Holds state for evaluating a starlark environment.
#[derive(Debug, Clone)]
pub struct EnvironmentContext {
    /// Directory the environment should be evaluated from.
    ///
    /// Typically used to resolve absolute paths to relative filenames.
    pub cwd: PathBuf,

    /// Logger to use for logging execution.
    pub logger: slog::Logger,

    /// Path to write distribution artifacts.
    pub dist_path: PathBuf,
}

/// Obtain a Starlark environment for evaluating distribution configuration.
pub fn global_environment(context: &EnvironmentContext) -> Result<Environment, EnvironmentError> {
    let env = Environment::new("global");

    let env = appdistribute_module(global_functions(env));
    let env = snap::snapcraft_module(env);

    // TODO perhaps capture these in a custom Environment type?
    env.set("CWD", Value::from(context.cwd.display().to_string()))?;
    env.set(
        "DIST_PATH",
        Value::from(context.dist_path.display().to_string()),
    )?;
    env.set("PIPELINES", List::new())?;

    Ok(env)
}
