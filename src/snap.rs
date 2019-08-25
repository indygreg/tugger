// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::filemanifest::FileManifest;
use serde::{Deserialize, Serialize};
use slog::{warn, Logger};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Represents a snapcraft.yaml part.* entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Vec<String>>,
    // TODO build-attributes
    #[serde(rename = "build-environment", skip_serializing_if = "Option::is_none")]
    pub build_environment: Option<HashMap<String, String>>,
    #[serde(rename = "build-packages", skip_serializing_if = "Option::is_none")]
    pub build_packages: Option<Vec<String>>,
    #[serde(rename = "build-snaps", skip_serializing_if = "Option::is_none")]
    pub build_snaps: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organize: Option<HashMap<String, String>>,
    #[serde(rename = "override-build", skip_serializing_if = "Option::is_none")]
    pub override_build: Option<String>,
    #[serde(rename = "override-prime", skip_serializing_if = "Option::is_none")]
    pub override_prime: Option<String>,
    #[serde(rename = "override-pull", skip_serializing_if = "Option::is_none")]
    pub override_pull: Option<String>,
    #[serde(rename = "override-stage", skip_serializing_if = "Option::is_none")]
    pub override_stage: Option<String>,
    #[serde(rename = "parse-info", skip_serializing_if = "Option::is_none")]
    pub parse_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "source-branch", skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<String>,
    #[serde(rename = "source-checksum", skip_serializing_if = "Option::is_none")]
    pub source_checksum: Option<String>,
    #[serde(rename = "source-commit", skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
    #[serde(rename = "source-depth", skip_serializing_if = "Option::is_none")]
    pub source_depth: Option<u64>,
    #[serde(rename = "source-subdir", skip_serializing_if = "Option::is_none")]
    pub source_subdir: Option<String>,
    #[serde(rename = "source-tag", skip_serializing_if = "Option::is_none")]
    pub source_tag: Option<String>,
    #[serde(rename = "source-type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Vec<String>>,
    #[serde(rename = "stage-packages", skip_serializing_if = "Option::is_none")]
    pub stage_packages: Option<Vec<String>>,
    #[serde(rename = "stage-snaps", skip_serializing_if = "Option::is_none")]
    pub stage_snaps: Option<Vec<String>>,
}

/// Represents a snapcraft.yaml app.* entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "command-chain", skip_serializing_if = "Option::is_none")]
    pub command_chain: Option<Vec<String>>,
    #[serde(rename = "commit-id", skip_serializing_if = "Option::is_none")]
    pub common_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,
    #[serde(rename = "listen-stream", skip_serializing_if = "Option::is_none")]
    pub listen_stream: Option<String>,
    // TODO passthrough
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugs: Option<Vec<String>>,
    #[serde(rename = "post-stop-command", skip_serializing_if = "Option::is_none")]
    pub post_stop_command: Option<String>,
    #[serde(rename = "restart-condition", skip_serializing_if = "Option::is_none")]
    pub restart_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket: Option<HashMap<String, String>>,
    #[serde(rename = "socket-mode", skip_serializing_if = "Option::is_none")]
    pub socket_mode: Option<u64>,
    #[serde(rename = "stop-command", skip_serializing_if = "Option::is_none")]
    pub stop_command: Option<String>,
    #[serde(rename = "stop-timeout", skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<String>,
}

/// Represents a snapcraft.yaml file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snap {
    // top-level metadata (https://snapcraft.io/docs/snapcraft-top-level-metadata).
    #[serde(rename = "adopt-info", skip_serializing_if = "Option::is_none")]
    pub adopt_info: Option<String>,
    // TODO architectures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confinement: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub name: String,
    // TODO passthrough
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugs: Option<HashMap<String, HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<HashMap<String, HashMap<String, String>>>,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "snap-type", skip_serializing_if = "Option::is_none")]
    pub snap_type: Option<String>,
    pub version: String,
    pub apps: HashMap<String, SnapApp>,
    pub parts: HashMap<String, SnapPart>,
}

/// Execute `snapcraft`.
///
/// `args` represents the arguments to the `snapcraft` executable.
/// `snap` represents the `snapcraft.yaml` file to create.
/// `build_path` is the directory to operate in.
/// `files` defines a manifest of files to constitute the build environment.
///
/// If `build_path` exists, its content will be replaced by the content of
/// `files`.
///
/// We need to provide an explicit and stable path to execute in because
/// snapcraft mounts the path into the build environment and isn't smart
/// enough to detect when the source path changes between invocations. By
/// using a stable path and swapping out the content from beneath snapcraft,
/// we work around the issue.
pub fn execute_snapcraft(
    logger: &Logger,
    args: &Vec<String>,
    snap: &Snap,
    build_path: &Path,
    files: &FileManifest,
) -> Result<(), String> {
    if !build_path.exists() {
        std::fs::create_dir_all(build_path)
            .or_else(|_| Err(format!("error creating {}", build_path.display())))?;
    }

    // Remove existing content of build directory and replace with our own.
    for entry in walkdir::WalkDir::new(build_path).contents_first(true) {
        let entry = entry.or_else(|_| Err("could not resolve directory entry".to_string()))?;
        let p = entry.path();

        if entry.path_is_symlink() || p.is_file() {
            std::fs::remove_file(p)
                .or_else(|_| Err(format!("unable to remove {}", p.display())))?;
        } else {
            std::fs::remove_dir(p).or_else(|_| Err(format!("unable to remove {}", p.display())))?;
        }
    }

    super::filemanifest::install_files(build_path, files);

    let snap_path = build_path.join("snap");
    std::fs::create_dir(&snap_path).expect("unable to create snap directory");
    let snapcraft_yaml_path = snap_path.join("snapcraft.yaml");

    let yaml = serde_yaml::to_vec(snap).expect("unable to format YAML");

    let mut fh = std::fs::File::create(&snapcraft_yaml_path).unwrap();
    fh.write_all(&yaml)
        .expect(&format!("unable to write {:?}", &snapcraft_yaml_path));

    let mut cmd = std::process::Command::new("snapcraft")
        .args(args)
        .current_dir(build_path)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("error running snapcraft");
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            warn!(logger, "{}", line.unwrap());
        }
    }

    Ok(())
}
