// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::EnvironmentContext;
use crate::starlark::values::Pipeline;
use codemap::CodeMap;
use codemap_diagnostic::{Diagnostic, Level};
use starlark::environment::Environment;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Represents the result of evaluating an environment.
pub struct EvalResult {
    /// The raw environment that was executed.
    pub env: Environment,
}

impl EvalResult {
    pub fn execute_all_pipelines(&self) -> Result<(), String> {
        let pipelines = self.env.get("PIPELINES").unwrap();

        let it = pipelines
            .into_iter()
            .or_else(|e| Err(format!("could not iterate PIPELINES: {:#?}", e)))?;

        for pv in it {
            let raw_value = pv.0.borrow();
            let pipeline: &Pipeline = raw_value.as_any().downcast_ref().unwrap();

            self.execute_raw_pipeline(pipeline)?;
        }

        Ok(())
    }

    /// Execute a defined pipeline.
    pub fn execute_pipeline(&self, name: &str) -> Result<(), String> {
        let pipelines = self.env.get("PIPELINES").unwrap();

        let it = pipelines
            .into_iter()
            .or_else(|e| Err(format!("could not iterate PIPELINES: {:#?}", e)))?;

        for pv in it {
            let raw_value = pv.0.borrow();
            let pipeline: &Pipeline = raw_value.as_any().downcast_ref().unwrap();

            if pipeline.name == name {
                return self.execute_raw_pipeline(pipeline);
            }
        }

        Err(format!("could not find pipeline {}", name))
    }

    fn execute_raw_pipeline(&self, pipeline: &Pipeline) -> Result<(), String> {
        println!("executing pipeline: {}", pipeline.name);
        for step in &pipeline.steps {
            println!("step: {:#?}", step);
        }

        Ok(())
    }
}

/// Evaluate an app distribution starlark file in the context of a current working directory.
pub fn evaluate_file(path: &Path, context: &EnvironmentContext) -> Result<EvalResult, Diagnostic> {
    let mut env = super::global_environment(context).or_else(|_| {
        Err(Diagnostic {
            level: Level::Error,
            message: "error creating environment".to_string(),
            code: Some("environment".to_string()),
            spans: vec![],
        })
    })?;

    let map = Arc::new(Mutex::new(CodeMap::new()));

    println!("evaluating {}", path.display());

    starlark::eval::simple::eval_file(&map, &path.display().to_string(), false, &mut env)?;

    Ok(EvalResult { env })
}
