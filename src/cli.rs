// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::starlark::eval::evaluate_file;
use super::starlark::EnvironmentContext;
use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;

pub fn run_cli() -> Result<(), String> {
    let matches = App::new("appdistribute")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .author("Gregory Szorc <gregory.szorc@gmail.com>")
        .long_about("Build distributable applications")
        .subcommand(
            SubCommand::with_name("repl")
                .about("Start an interactive REPL to evaluate build rules"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Evaluate a build file")
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .value_name("PATH")
                        .help("Path to file to evaluate"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("repl", Some(_)) => {
            let cwd = std::env::current_dir().unwrap();
            let context = EnvironmentContext { cwd };
            let env = super::starlark::global_environment(&context)
                .or_else(|_| Err(String::from("error creating environment")))?;

            starlark_repl::repl(&env, false);

            Ok(())
        }
        ("run", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let path = PathBuf::from(path);

            let context = EnvironmentContext {
                cwd: path.parent().unwrap().to_path_buf(),
            };

            let eval_result = match evaluate_file(&path, &context) {
                Ok(res) => {
                    println!("evaluation complete");
                    Ok(res)
                }
                Err(e) => Err(format!("error evaluating {}: {:#?}", path.display(), e)),
            }?;

            eval_result.execute_all_pipelines()
        }
        _ => Err("invalid sub-command".to_string()),
    }
}
