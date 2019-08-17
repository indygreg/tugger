// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use starlark::environment::{Environment, EnvironmentError};
use starlark::stdlib::global_functions;
use starlark::values::list::List;
use starlark::values::{Value, ValueError, ValueResult};
use starlark::{
    starlark_fun, starlark_module, starlark_signature, starlark_signature_extraction,
    starlark_signatures,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

mod values;

use values::SourceFile;

fn evaluate_glob(cwd: &str, pattern: &str) -> Vec<PathBuf> {
    let search = if pattern.starts_with('/') {
        pattern.to_string()
    } else {
        format!("{}/{}", cwd, pattern)
    };

    let mut res = Vec::new();

    for path in glob::glob(&search).unwrap() {
        res.push(path.unwrap());
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
}

/// Obtain a Starlark environment for evaluating distribution configuration.
pub fn global_environment(cwd: &Path) -> Result<Environment, EnvironmentError> {
    let env = Environment::new("global");

    let env = appdistribute_module(global_functions(env));

    env.set("CWD", Value::from(cwd.display().to_string()))?;

    Ok(env)
}
