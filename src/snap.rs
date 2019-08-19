// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a snapcraft.yaml part.* entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapPart {
    pub after: Option<Vec<String>>,
    // TODO build-attributes
    #[serde(rename = "build-environment")]
    pub build_environment: Option<HashMap<String, String>>,
    #[serde(rename = "build-packages")]
    pub build_packages: Option<Vec<String>>,
    #[serde(rename = "build-snaps")]
    pub build_snaps: Option<Vec<String>>,
    pub filesets: Option<Vec<String>>,
    pub organize: Option<HashMap<String, String>>,
    #[serde(rename = "override-build")]
    pub override_build: Option<String>,
    #[serde(rename = "override-prime")]
    pub override_prime: Option<String>,
    #[serde(rename = "override-pull")]
    pub override_pull: Option<String>,
    #[serde(rename = "override-stage")]
    pub override_stage: Option<String>,
    #[serde(rename = "parse-info")]
    pub parse_info: Option<String>,
    pub plugin: Option<String>,
    pub prime: Option<Vec<String>>,
    pub source: Option<String>,
    #[serde(rename = "source-branch")]
    pub source_branch: Option<String>,
    #[serde(rename = "source-checksum")]
    pub source_checksum: Option<String>,
    #[serde(rename = "source-commit")]
    pub source_commit: Option<String>,
    #[serde(rename = "source-depth")]
    pub source_depth: Option<u64>,
    #[serde(rename = "source-subdir")]
    pub source_subdir: Option<String>,
    #[serde(rename = "source-tag")]
    pub source_tag: Option<String>,
    #[serde(rename = "source-type")]
    pub source_type: Option<String>,
    pub stage: Option<Vec<String>>,
    #[serde(rename = "stage-packages")]
    pub stage_packages: Option<Vec<String>>,
    #[serde(rename = "stage-snaps")]
    pub stage_snaps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapApp {
    pub adapter: Option<String>,
    pub command: Option<String>,
    #[serde(rename = "command-chain")]
    pub command_chain: Option<Vec<String>>,
    #[serde(rename = "commit-id")]
    pub common_id: Option<String>,
    pub daemon: Option<String>,
    pub desktop: Option<String>,
    pub environment: Option<HashMap<String, String>>,
    #[serde(rename = "listen-stream")]
    pub listen_stream: Option<String>,
    // TODO passthrough
    pub plugs: Option<Vec<String>>,
    #[serde(rename = "post-stop-command")]
    pub post_stop_command: Option<String>,
    #[serde(rename = "restart-condition")]
    pub restart_condition: Option<String>,
    pub slots: Option<Vec<String>>,
    pub socket: Option<HashMap<String, String>>,
    #[serde(rename = "socket-mode")]
    pub socket_mode: Option<u64>,
    #[serde(rename = "stop-command")]
    pub stop_command: Option<String>,
    #[serde(rename = "stop-timeout")]
    pub stop_timeout: Option<String>,
}

/// Represents a snapcraft.yaml file content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snap {
    // top-level metadata (https://snapcraft.io/docs/snapcraft-top-level-metadata).
    #[serde(rename = "adopt-info")]
    pub adopt_info: Option<String>,
    // TODO architectures
    pub assumes: Option<Vec<String>>,
    pub base: Option<String>,
    pub confinement: Option<String>,
    pub description: String,
    pub grade: Option<String>,
    pub icon: Option<String>,
    pub license: Option<String>,
    pub name: String,
    // TODO passthrough
    pub plugs: Option<HashMap<String, HashMap<String, String>>>,
    pub slots: Option<HashMap<String, HashMap<String, String>>>,
    pub summary: String,
    pub title: Option<String>,
    #[serde(rename = "snap-type")]
    pub snap_type: Option<String>,
    pub version: String,
    pub apps: HashMap<String, SnapApp>,
    pub parts: HashMap<String, SnapPart>,
}
