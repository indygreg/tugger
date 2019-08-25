// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/*!
The `starlark` module and related sub-modules define the
[Starlark](https://github.com/bazelbuild/starlark) dialect used to power
`tugger` configuration files and provide mechanisms for evaluating those
files.

The custom Starlark primitives provided by the dialect are documented
in the sections below.

## File Representation and Manipulation

### `SourceFile`

Type used to represent a file.

Instances are typically constructed by other functions.

### `FileManifest`

Type representing a virtual filesystem mapping of relative filenames to
file content. `FileManifest` instances are used to represent things like
file layouts in an installed directory, lists of files to package, etc.

Instances are typically constructed by other functions.

### `glob(include, exclude=None)`

Resolve file patterns to files.

`include` is a `str` or `list` of `str` containing filenames that will be
matched using the `glob` Rust crate. If filenames begin with `/` they are
absolute. Otherwise they are relative to the directory the file is
being evaluated in.

`exclude` has the same type as `include` but is used to exclude certain
files from the result. All patterns in `include` are evaluated before
`exclude`.

Returns a `list` of `SourceFile` instances.

### `file_manifest_from_files(files, relative_to=None, prefix=None)`

Construct a `FileManifest` from an iterable of `SourceFile` instances.

The paths in `FileManifest` will be relative to the `relative_to` path,
which by default is the relative directory of the Starlark file currently
being evaluated.

`prefix` can be used to prefix all relative paths with a value.

It is common to pass the output of `glob()` as the value for the `files`
argument.

## Pipelines

Pipelines are an entity with a name and a series of steps to execute.

### `Pipeline`

Represents a constructed pipeline. Instances are produced by calling the
`pipeline()` function.

### `pipeline(name, steps=[])`

Create a pipeline from a series of steps.

`name` is the unique name of this pipeline. It will be displayed during
processing.

`steps` is a list of objects that are known `actions`/`steps` types.

## Actions

Actions represent a logically discrete unit of work. They are the building
blocks of `pipelines`.

Actions are created by calling functions that define an action. These
functions are described below.

### `snapcraft(args, snap, build_path, manifest)`

Define an invocation of `snapcraft`.

`snapcraft` is a packaging tool used to produce and interface with snaps.
This function produces an action that will translate to invocation of the
`snapcraft` command.

The `args` argument is a `list` of `str` defining arguments to the
`snapcraft` command.

The `snap` argument is a `Snap` instance. See the `snap()` function for
how to create one.

`build_path` is the path to use when invoking `snapcraft`. If the path
exists, its contents will be replaced by the content of `manifest`.

`manifest` is a `FileManifest` for the snap build environment. Then
`snapcraft` is invoked, it will be done so from a temporary directory
composed of the files defined by this manifest.

Having to define `build_path` is a bit unfortunate. But various `snapcraft`
behavior relies on mounting a local filesystem path into a virtual machine,
container, etc, and `snapcraft` isn't smart enough to realize that the source
directory changed between invocations. By providing a consistent path between
invocations, we can work around this behavior.

### `tar_archive(filename, manifest)`

Produce a tar archive from a manifest of files.

`filename` is a string denoting the output filename.

`manifest` is a `FileManifest` describing the files to add to the archive.
The value will be copied and modifications to the original `FileManifest`
will not be reflected on the returned instance.

Returns a `TarArchive` describing a tar archive to produce.

## Snapcraft Configuration

Various types and functions exist to define a `snapcraft.yaml`
configuration file.

### `snap_part(**kwargs)`

This function returns a `SnapPart` type which will represents a
`part` entry in a `snapcraft.yaml` file. Arguments to this function
are the various attributes that can exist in
[snapcraft parts metadata](https://snapcraft.io/docs/snapcraft-parts-metadata).
`-` in key names is replaced by `_` in the argument name. For example,
the `override-build` key would be defined by the `override_build`
argument.

### `snap_app(**kwargs)`

This function returns a `SnapApp` type which represents an
`apps` entry in a `snapcraft.yaml` file. Arguments to this function
are the various attributes that can exist in
[snapcraft app and service metadata](https://snapcraft.io/docs/snapcraft-app-and-service-metadata).
`-` in key names is replaced by `_` in the argument name. For example,
the `commid-id` key would be defined by the `common_id` argument.

### `snap(name, description, summary, version, **kwargs)`

This function returns a `Snap` type which represents a full
`snapcraft.yaml` file. Arguments to this function are the various
attributes that can exist in
[snapcraft top-level metadata](https://snapcraft.io/docs/snapcraft-top-level-metadata).
`-` in key names is replaced by `_` in the argument name. For example,
the `snap-type` key would be defined by the `snap_type` argument.

*/

use super::glob::evaluate_glob;
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
pub mod snap;
pub mod values;

use values::{FileManifest, Pipeline, SourceFile, Step, TarArchive};

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
                if k.get_type() == key_type {
                    Ok(())
                } else {
                    Err(RuntimeError {
                        code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
                        message: format!(
                            "dict {} expects keys of type {}; got {}",
                            arg_name,
                            key_type,
                            k.get_type()
                        ),
                        label: format!("expected type {}; got {}", key_type, k.get_type()),
                    }
                    .into())
                }?;

                let v = value.at(k.clone())?;

                if v.get_type() == value_type {
                    Ok(())
                } else {
                    Err(RuntimeError {
                        code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
                        message: format!(
                            "dict {} expects values of type {}; got {}",
                            arg_name,
                            value_type,
                            v.get_type(),
                        ),
                        label: format!("expected type {}; got {}", value_type, v.get_type()),
                    }
                    .into())
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

fn required_list_arg(arg_name: &str, value_type: &str, value: &Value) -> Result<(), ValueError> {
    match value.get_type() {
        "list" => {
            for v in value.into_iter()? {
                if v.get_type() == value_type {
                    Ok(())
                } else {
                    Err(RuntimeError {
                        code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
                        message: format!(
                            "list {} expects values of type {}; got {}",
                            arg_name,
                            value_type,
                            v.get_type()
                        ),
                        label: format!("expected type {}; got {}", value_type, v.get_type()),
                    }
                    .into())
                }?;
            }
            Ok(())
        }
        t => Err(RuntimeError {
            code: INCORRECT_PARAMETER_TYPE_ERROR_CODE,
            message: format!("function expects a list for {}; got type {}", arg_name, t),
            label: format!("expected type list; got {}", t),
        }
        .into()),
    }
}

starlark_module! { tugger_module =>
    glob(env env, include, exclude=None) {
        let cwd = env.get("CWD").unwrap().to_str();

        resolve_include_exclude(&cwd, &include, &exclude)
    }

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
                "Snapcraft" => {
                    let raw_value = step.0.borrow();
                    let snapcraft: &snap::Snapcraft = raw_value.as_any().downcast_ref().unwrap();
                    Step::Snapcraft(snapcraft.clone())
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

    let env = tugger_module(global_functions(env));
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
