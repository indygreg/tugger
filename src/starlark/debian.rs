// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use debian::package::ControlParagraph;

use super::{
    optional_list_arg, optional_str_arg, required_list_arg, required_str_arg, required_type_arg,
};
use crate::starlark::values::FileManifest;
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
pub struct DebianControlSourceBinaryPackage {
    pub paragraph: ControlParagraph,
}

impl TypedValue for DebianControlSourceBinaryPackage {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("DebianControlSourceBinaryPackage<{:#?}", self.paragraph)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "DebianControlSourceBinaryPackage"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct DebianControl {
    pub paragraphs: Vec<ControlParagraph>,
}

impl TypedValue for DebianControl {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("DebianControl<{:#?}", self.paragraphs)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "DebianControl"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct DebianControlBinaryPackage {
    pub paragraph: ControlParagraph,
}

impl TypedValue for DebianControlBinaryPackage {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!("DebianControlBinaryPackage<{:#?}", self.paragraph)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "DebianControlBinaryPackage"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct DebianDebArchive {
    pub control_file: DebianControlBinaryPackage,
    pub files: FileManifest,
}

impl TypedValue for DebianDebArchive {
    immutable!();
    any!();
    not_supported!(binop);
    not_supported!(container);
    not_supported!(function);
    not_supported!(get_hash);
    not_supported!(to_int);

    fn to_str(&self) -> String {
        format!(
            "DebianDebArchive<control_file={:#?}, files={:#?}",
            self.control_file, self.files
        )
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "DebianDebArchive"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }
}

fn str_list_to_comma_string(value: &Value) -> String {
    let strings: Vec<String> = value.into_iter().unwrap().map(|x| x.to_string()).collect();

    strings.join(", ")
}

starlark_module! { debian_module =>
    debian_control_source_binary_package(
        package,
        architecture,
        description,
        section=None,
        priority=None,
        essential=None,
        homepage=None,
        built_using=None,
        package_type=None,
        depends=None,
        pre_depends=None,
        recommends=None,
        suggests=None,
        enhances=None,
        breaks=None,
        conflicts=None) {

        required_str_arg("package", &package)?;
        required_str_arg("architecture", &architecture)?;
        required_str_arg("description", &description)?;
        optional_str_arg("section", &section)?;
        optional_str_arg("priority", &priority)?;
        optional_str_arg("essential", &essential)?;
        optional_str_arg("homepage", &homepage)?;
        optional_str_arg("built_using", &built_using)?;
        optional_str_arg("package_type", &package_type)?;
        optional_list_arg("depends", "string", &depends)?;
        optional_list_arg("pre_depends", "string", &pre_depends)?;
        optional_list_arg("recommends", "string", &recommends)?;
        optional_list_arg("suggests", "string", &suggests)?;
        optional_list_arg("enhances", "string", &enhances)?;
        optional_list_arg("breaks", "string", &breaks)?;
        optional_list_arg("conflicts", "string", &conflicts)?;

        let mut paragraph = ControlParagraph::new();
        paragraph.add_entry("Package", package.to_string());
        paragraph.add_entry("Architecture", architecture.to_string());
        paragraph.add_entry("Description", description.to_string());

        if section.get_type() != "NoneType" {
            paragraph.add_entry("Section", section.to_string());
        }
        if priority.get_type() != "NoneType" {
            paragraph.add_entry("Priority", priority.to_string());
        }
        if essential.get_type() != "NoneType" {
            paragraph.add_entry("Essential", essential.to_string());
        }
        if homepage.get_type() != "NoneType" {
            paragraph.add_entry("Homepage", homepage.to_string());
        }
        if built_using.get_type() != "NoneType" {
            paragraph.add_entry("Built-Using", built_using.to_string());
        }
        if package_type.get_type() != "NoneType" {
            paragraph.add_entry("Package-Type", package_type.to_string());
        }
        if depends.get_type() != "NoneType" {
            paragraph.add_entry("Depends", str_list_to_comma_string(&depends));
        }
        if pre_depends.get_type() != "NoneType" {
            paragraph.add_entry("Pre-Depends", str_list_to_comma_string(&pre_depends));
        }
        if recommends.get_type() != "NoneType" {
            paragraph.add_entry("Recommends", str_list_to_comma_string(&recommends));
        }
        if suggests.get_type() != "NoneType" {
            paragraph.add_entry("Suggests", str_list_to_comma_string(&suggests));
        }
        if enhances.get_type() != "NoneType" {
            paragraph.add_entry("Enhances", str_list_to_comma_string(&enhances));
        }
        if breaks.get_type() != "NoneType" {
            paragraph.add_entry("Breaks", str_list_to_comma_string(&breaks));
        }
        if conflicts.get_type() != "NoneType" {
            paragraph.add_entry("Conflicts", str_list_to_comma_string(&conflicts));
        }

        Ok(Value::new(DebianControlSourceBinaryPackage { paragraph }))
    }

    debian_control(
        source,
        maintainer,
        standards_version,
        uploaders=None,
        section=None,
        priority=None,
        build_depends=None,
        homepage=None,
        vcs_type=None,
        vcs_value=None,
        vcs_browser=None,
        rules_requires_root=None,
        binary_packages=None) {

        required_str_arg("source", &source)?;
        required_str_arg("maintainer", &maintainer)?;
        required_str_arg("standards_version", &standards_version)?;
        optional_list_arg("uploaders", "string", &uploaders)?;
        optional_str_arg("section", &section)?;
        optional_str_arg("priority", &priority)?;
        optional_list_arg("build_depends", "string", &build_depends)?;
        optional_str_arg("homepage", &homepage)?;
        optional_str_arg("vcs_type", &vcs_type)?;
        optional_str_arg("vcs_value", &vcs_value)?;
        optional_str_arg("vcs_browser", &vcs_browser)?;
        optional_str_arg("rules_requires_root", &rules_requires_root)?;
        required_list_arg("binary_packages", "DebianControlSourceBinaryPackage", &binary_packages)?;

        let mut source_paragraph = ControlParagraph::new();
        source_paragraph.add_entry("Source", source.to_string());
        source_paragraph.add_entry("Maintainer", maintainer.to_string());
        source_paragraph.add_entry("Standards-Version", standards_version.to_string());

        if uploaders.get_type() != "NoneType" {
            source_paragraph.add_entry("Uploaders", str_list_to_comma_string(&uploaders));
        }
        if section.get_type() != "NoneType" {
            source_paragraph.add_entry("Section", section.to_string());
        }
        if priority.get_type() != "NoneType" {
            source_paragraph.add_entry("Priority", priority.to_string());
        }
        if build_depends.get_type() != "NoneType" {
            source_paragraph.add_entry("Build-Depends", str_list_to_comma_string(&build_depends));
        }
        if homepage.get_type() != "NoneType" {
            source_paragraph.add_entry("Homepage", homepage.to_string());
        }
        if vcs_type.get_type() != "NoneType" && vcs_value.get_type() != "NoneType" {
            let key = format!("Vcs-{}", vcs_type.to_string());
            source_paragraph.add_entry(&key, vcs_value.to_string());
        }
        if vcs_browser.get_type() != "NoneType" {
            source_paragraph.add_entry("Vcs-Browser", vcs_browser.to_string());
        }
        if rules_requires_root.get_type() != "NoneType" {
            source_paragraph.add_entry("Rules-Requires-Root", rules_requires_root.to_string());
        }

        let mut paragraphs = Vec::new();
        paragraphs.push(source_paragraph);

        for package in binary_packages.into_iter()? {
            let raw_package = package.0.borrow();
            let package: &DebianControlSourceBinaryPackage = raw_package.as_any().downcast_ref().unwrap();
            paragraphs.push(package.paragraph.clone());
        }

        Ok(Value::new(DebianControl { paragraphs }))
    }

    debian_control_binary_package(
        package,
        version,
        architecture,
        maintainer,
        description,
        source=None,
        section=None,
        priority=None,
        essential=None,
        depends=None,
        pre_depends=None,
        recommends=None,
        suggests=None,
        enhances=None,
        breaks=None,
        conflicts=None,
        installed_size=None,
        homepage=None,
        built_using=None) {

        let package = required_str_arg("package", &package)?;
        let version = required_str_arg("version", &version)?;
        let architecture = required_str_arg("architecture", &architecture)?;
        let maintainer = required_str_arg("maintainer", &maintainer)?;
        let description = required_str_arg("description", &description)?;
        let source = optional_str_arg("source", &source)?;
        let section = optional_str_arg("section", &section)?;
        let priority = optional_str_arg("priority", &priority)?;
        let essential = optional_str_arg("essential", &essential)?;

        optional_list_arg("depends", "string", &depends)?;
        optional_list_arg("pre_depends", "string", &pre_depends)?;
        optional_list_arg("recommends", "string", &recommends)?;
        optional_list_arg("suggests", "string", &suggests)?;
        optional_list_arg("enhances", "string", &enhances)?;
        optional_list_arg("breaks", "string", &breaks)?;
        optional_list_arg("conflicts", "string", &conflicts)?;

        let installed_size = optional_str_arg("installed_size", &installed_size)?;
        let homepage = optional_str_arg("homepage", &homepage)?;
        let built_using = optional_str_arg("built_using", &built_using)?;

        let mut paragraph = ControlParagraph::new();
        paragraph.add_entry("Package", package);
        paragraph.add_entry("Version", version);
        paragraph.add_entry("Architecture", architecture);
        paragraph.add_entry("Maintainer", maintainer);
        paragraph.add_entry("Description", description);

        if let Some(source) = source {
            paragraph.add_entry("Source", source);
        }
        if let Some(section) = section {
            paragraph.add_entry("Section", section);
        }
        if let Some(priority) = priority {
            paragraph.add_entry("Priority", priority);
        }
        if let Some(essential) = essential {
            paragraph.add_entry("Essential", essential);
        }
        if depends.get_type() != "NoneType" {
            paragraph.add_entry("Depends", str_list_to_comma_string(&depends));
        }
        if pre_depends.get_type() != "NoneType" {
            paragraph.add_entry("Pre-Depends", str_list_to_comma_string(&pre_depends));
        }
        if recommends.get_type() != "NoneType" {
            paragraph.add_entry("Recommends", str_list_to_comma_string(&recommends));
        }
        if suggests.get_type() != "NoneType" {
            paragraph.add_entry("Suggests", str_list_to_comma_string(&suggests));
        }
        if enhances.get_type() != "NoneType" {
            paragraph.add_entry("Enhances", str_list_to_comma_string(&enhances));
        }
        if breaks.get_type() != "NoneType" {
            paragraph.add_entry("Breaks", str_list_to_comma_string(&breaks));
        }
        if conflicts.get_type() != "NoneType" {
            paragraph.add_entry("Conflicts", str_list_to_comma_string(&conflicts));
        }
        if let Some(installed_size) = installed_size {
            paragraph.add_entry("Installed-Size", installed_size);
        }
        if let Some(homepage) = homepage {
            paragraph.add_entry("Homepage", homepage);
        }
        if let Some(built_using) = built_using {
            paragraph.add_entry("Built-Using", built_using);
        }

        Ok(Value::new(DebianControlBinaryPackage { paragraph }))
    }

    debian_deb_archive(control_binary_package, files) {
        required_type_arg("control_binary_package", "DebianControlBinaryPackage", &control_binary_package)?;
        required_type_arg("files", "FileManifest", &files)?;

        let raw_package = control_binary_package.0.borrow();
        let package: &DebianControlBinaryPackage = raw_package.as_any().downcast_ref().unwrap();
        let raw_manifest = files.0.borrow();
        let manifest: &FileManifest = raw_manifest.as_any().downcast_ref().unwrap();

        Ok(Value::new(DebianDebArchive {
            control_file: package.clone(),
            files: manifest.clone(),
        }))
    }
}
