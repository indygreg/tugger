// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{optional_str_arg, required_dict_arg, required_str_arg};
use starlark::environment::Environment;
use starlark::starlark_module;
use starlark::values::{default_compare, TypedValue, Value, ValueError, ValueResult};
use starlark::{
    any, immutable, not_supported, starlark_fun, starlark_signature, starlark_signature_extraction,
    starlark_signatures,
};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Snap {
    pub snap: crate::snap::Snap,
}

impl TypedValue for Snap {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("Snap<{:#?}>", self.snap)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "Snap"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct SnapPart {
    pub part: crate::snap::SnapPart,
}

impl TypedValue for SnapPart {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("SnapPart<{:#?}>", self.part)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "SnapPart"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct SnapApp {
    pub app: crate::snap::SnapApp,
}

impl TypedValue for SnapApp {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("SnapApp<{:#?}>", self.app)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "SnapApp"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

starlark_module! { snapcraft_module =>
    snap_part(after=None, build_environment=None, build_packages=None, build_snaps=None,
              filesets=None, organize=None, override_build=None, override_prime=None,
              override_pull=None, override_stage=None, parse_info=None, plugin=None, prime=None,
              source=None, source_branch=None, source_checksum=None, source_commit=None,
              source_depth=None, source_subdir=None, source_tag=None, source_type=None, stage=None,
              stage_packages=None, stage_snaps=None) {

        let part = crate::snap::SnapPart {
            after: None,
            build_environment: None,
            build_packages: None,
            build_snaps: None,
            filesets: None,
            organize: None,
            override_build: optional_str_arg("override_build", &override_build)?,
            override_prime: optional_str_arg("override_prime", &override_prime)?,
            override_pull: optional_str_arg("override_pull", &override_pull)?,
            override_stage: optional_str_arg("override_stage", &override_stage)?,
            parse_info: optional_str_arg("parse_info", &parse_info)?,
            plugin: optional_str_arg("plugin", &plugin)?,
            prime: None,
            source: optional_str_arg("source", &source)?,
            source_branch: optional_str_arg("source_branch", &source_branch)?,
            source_checksum: optional_str_arg("source_checksum", &source_checksum)?,
            source_commit: optional_str_arg("source_commit", &source_commit)?,
            source_depth: None,
            source_subdir: optional_str_arg("source_subdir", &source_subdir)?,
            source_tag: optional_str_arg("source_tag", &source_tag)?,
            source_type: optional_str_arg("source_type", &source_type)?,
            stage: None,
            stage_packages: None,
            stage_snaps: None,
        };

        Ok(Value::new(SnapPart { part }))
    }

    snap_app(adapter=None, command=None, command_chain=None, common_id=None,
             daemon=None, desktop=None, environment=None, listen_stream=None,
             plugs=None, post_stop_command=None, restart_condition=None, slots=None,
             socket=None, socket_module=None, stop_command=None, stop_timeout=None) {

        let app = crate::snap::SnapApp {
            adapter: optional_str_arg("adapter", &adapter)?,
            command: optional_str_arg("command", &command)?,
            command_chain: None,
            common_id: optional_str_arg("common_id", &common_id)?,
            daemon: optional_str_arg("daemon", &daemon)?,
            desktop: optional_str_arg("desktop", &desktop)?,
            environment: None,
            listen_stream: optional_str_arg("listen_stream", &listen_stream)?,
            plugs: None,
            post_stop_command: optional_str_arg("post_stop_command", &post_stop_command)?,
            restart_condition: optional_str_arg("restart_condition", &restart_condition)?,
            slots: None,
            socket: None,
            socket_mode: None,
            stop_command: optional_str_arg("stop_command", &stop_command)?,
            stop_timeout: optional_str_arg("stop_timeout", &stop_timeout)?,
        };

        Ok(Value::new(SnapApp { app }))
    }

    snap(name, description, summary, version, adopt_info=None, assumes=None, base=None,
         confinement=None, grade=None, icon=None, license=None, plugs=None, slots=None,
         title=None, snap_type=None, parts=None, apps=None) {

        required_dict_arg("apps", "string", "SnapApp", &apps)?;
        required_dict_arg("parts", "string", "SnapPart", &parts)?;

        let mut raw_apps = HashMap::new();

        for k in apps.into_iter()? {
            let v = apps.at(k.clone())?;

            let raw_value = v.0.borrow();
            let snap_app: &SnapApp = raw_value.as_any().downcast_ref().unwrap();
            raw_apps.insert(k.to_str(), snap_app.app.clone());
        }

        let mut raw_parts = HashMap::new();

        for k in parts.into_iter()? {
            let v = parts.at(k.clone())?;

            let raw_value = v.0.borrow();
            let snap_part: &SnapPart = raw_value.as_any().downcast_ref().unwrap();
            raw_parts.insert(k.to_str(), snap_part.part.clone());
        }

        let snap = crate::snap::Snap {
            adopt_info: optional_str_arg("adopt_info", adopt_info)?,
            assumes: None,
            base: optional_str_arg("base", base)?,
            confinement: optional_str_arg("confinement", confinement)?,
            description: required_str_arg("description", description)?,
            grade: optional_str_arg("grade", grade)?,
            icon: optional_str_arg("icon", icon)?,
            license: optional_str_arg("license", license)?,
            name: required_str_arg("name", name)?,
            plugs: None,
            slots: None,
            summary: required_str_arg("summary", summary)?,
            title: optional_str_arg("title", title)?,
            snap_type: optional_str_arg("snap_type", snap_type)?,
            version: required_str_arg("version", version)?,
            apps: raw_apps,
            parts: raw_parts,
        };

        Ok(Value::new(Snap { snap }))
    }
}