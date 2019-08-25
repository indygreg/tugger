// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::starlark::eval::evaluate_file;
use super::starlark::EnvironmentContext;
use clap::{App, AppSettings, Arg, SubCommand};
use std::path::PathBuf;

use slog::warn;
use slog::Drain;

pub struct PrintlnDrain {
    min_level: slog::Level,
}

impl Drain for PrintlnDrain {
    type Ok = ();
    type Err = std::io::Error;

    fn log(
        &self,
        record: &slog::Record,
        _values: &slog::OwnedKVList,
    ) -> Result<Self::Ok, Self::Err> {
        if record.level().is_at_least(self.min_level) {
            println!("{}", record.msg());
        }

        Ok(())
    }
}

pub fn run_cli() -> Result<(), String> {
    let matches = App::new("tugger")
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
                        .value_name("PATH")
                        .default_value("tugger.ship")
                        .help("Path to file to evaluate"),
                ),
        )
        .get_matches();

    let logger = slog::Logger::root(
        PrintlnDrain {
            min_level: slog::Level::Info,
        }
        .fuse(),
        slog::o!(),
    );

    let cwd = std::env::current_dir().unwrap();
    let dist_path = cwd.join("dist");

    match matches.subcommand() {
        ("repl", Some(_)) => {
            let context = EnvironmentContext {
                cwd,
                logger,
                dist_path,
            };
            let env = super::starlark::global_environment(&context)
                .or_else(|_| Err(String::from("error creating environment")))?;

            starlark_repl::repl(&env, false);

            Ok(())
        }
        ("run", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let path = PathBuf::from(path);

            let normalized = path.canonicalize().unwrap();

            let context = EnvironmentContext {
                cwd: normalized.parent().unwrap().to_path_buf(),
                logger: logger.clone(),
                dist_path,
            };

            let eval_result = match evaluate_file(&path, &context) {
                Ok(res) => {
                    warn!(logger, "evaluation complete");
                    Ok(res)
                }
                Err(e) => Err(format!("error evaluating {}: {:#?}", path.display(), e)),
            }?;

            eval_result.execute_all_pipelines()
        }
        _ => Err("invalid sub-command".to_string()),
    }
}
