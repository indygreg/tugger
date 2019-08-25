// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::filemanifest::FileManifest;
use ar::{Builder, Header};
use debian::package::ControlFile;
use is_executable::IsExecutable;
use slog::{warn, Logger};
use std::io::Write;
use std::path::Path;
use tar::Header as TarHeader;

/// Produce data for a .deb package file.
///
/// The raw .deb data will be written to `writer`.
///
/// The installed files are defined by `files`.
pub fn build_deb<W>(
    writer: W,
    control_file: &ControlFile,
    files: &FileManifest,
) -> Result<(), String>
where
    W: Write,
{
    // The file format is documented at https://manpages.debian.org/unstable/dpkg-dev/deb.5.en.html.
    let mut ar_builder = Builder::new(writer);

    let system_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .or_else(|_| Err("could not compute duration".to_string()))?
        .as_secs();

    // First entry is a debian-binary file with static content.
    let data: &[u8] = b"2.0\n";
    let mut header = Header::new("debian-binary".as_bytes().to_owned(), data.len() as u64);
    header.set_mode(0o644);
    header.set_mtime(system_time);
    header.set_uid(0);
    header.set_gid(0);
    ar_builder
        .append(&header, data)
        .or_else(|e| Err(e.to_string()))?;

    // Second entry is a control.tar with metadata.
    let mut control_tar: Vec<u8> = Vec::new();
    build_control_tar(&mut control_tar, control_file, files, system_time)?;

    let mut header = Header::new(
        "control.tar".as_bytes().to_owned(),
        control_tar.len() as u64,
    );
    header.set_mode(0o644);
    header.set_mtime(system_time);
    header.set_uid(0);
    header.set_gid(0);
    ar_builder
        .append(&header, &control_tar as &[u8])
        .or_else(|e| Err(e.to_string()))?;

    // Third entry is a data.tar with file content.
    let mut data_tar: Vec<u8> = Vec::new();
    build_data_tar(&mut data_tar, files, system_time)?;

    // TODO compress data.

    let mut header = Header::new("data.tar".as_bytes().to_owned(), data_tar.len() as u64);
    header.set_mode(0o644);
    header.set_mtime(system_time);
    header.set_uid(0);
    header.set_gid(0);
    ar_builder
        .append(&header, &data_tar as &[u8])
        .or_else(|e| Err(e.to_string()))?;

    Ok(())
}

/// Build tar data stream for a control.tar file embedded in a .deb archive.
pub fn build_control_tar<W>(
    writer: W,
    control_file: &ControlFile,
    files: &FileManifest,
    mtime: u64,
) -> Result<(), String>
where
    W: Write,
{
    let control_data = serialize_control_file(control_file)?;
    let md5sums =
        make_md5sums(files).or_else(|e| Err(format!("unable to compute md5sums: {}", e)))?;

    let mut builder = tar::Builder::new(writer);

    let mut header = TarHeader::new_gnu();
    header.set_mtime(mtime);
    header.set_path("control").unwrap();
    header.set_mode(0o644);
    header.set_size(control_data.len() as u64);
    header.set_cksum();
    builder
        .append(&header, &control_data as &[u8])
        .or_else(|e| Err(format!("unable to append control: {}", e)))?;

    let mut header = TarHeader::new_gnu();
    header.set_mtime(mtime);
    header.set_path("md5sums").unwrap();
    header.set_mode(0o644);
    header.set_size(md5sums.len() as u64);
    header.set_cksum();
    builder
        .append(&header, &md5sums as &[u8])
        .or_else(|e| Err(format!("unable to append md5sums: {}", e)))?;

    // We could also support maintainer scripts. For another day...

    builder
        .finish()
        .or_else(|e| Err(format!("unable to finish archive: {}", e)))?;

    Ok(())
}

/// Generate the file content for an md5sums file in a control.tar archive.
pub fn make_md5sums(files: &FileManifest) -> Result<Vec<u8>, std::io::Error> {
    let mut res = Vec::new();

    for (rel_path, source_path) in files.iter() {
        let file_content = std::fs::read(&source_path)?;
        let digest = md5::compute(&file_content);

        res.write_all(&digest.to_ascii_lowercase())?;
        res.write_all(b"  ")?;
        res.write_all(rel_path.as_bytes())?;
        res.write_all(&[b'\n'])?;
    }

    Ok(res)
}

/// Obtain the file content of a Debian control file.
///
/// We need this wrapper because the serialization logic in ControlFile insists on
/// using a file.
pub fn serialize_control_file(control_file: &ControlFile) -> Result<Vec<u8>, String> {
    let tf = tempfile::NamedTempFile::new()
        .or_else(|e| Err(format!("unable to create temp file: {}", e)))?;
    {
        control_file
            .serialize(tf.path())
            .or_else(|e| Err(format!("unable to serialize Debian control file: {}", e)))?;
        let data = std::fs::read(tf.path())
            .or_else(|e| Err(format!("unable to read temp file: {}", e)))?;

        Ok(data)
    }
}

/// Build tar data stream for a data.tar file in a .deb archive.
pub fn build_data_tar<W>(writer: W, files: &FileManifest, mtime: u64) -> Result<(), String>
where
    W: Write,
{
    let mut builder = tar::Builder::new(writer);

    // TODO add directories.

    // We use a BTreeMap, so this should be deterministic.
    for (rel_path, source_path) in files.iter() {
        let file_content = std::fs::read(&source_path).or_else(|e| {
            Err(format!(
                "error reading file {}: {}",
                source_path.display(),
                e
            ))
        })?;

        let mut header = TarHeader::new_gnu();
        header.set_mtime(mtime);
        header.set_path(format!("./{}", rel_path)).unwrap();
        header.set_mode(if source_path.is_executable() {
            0o755
        } else {
            0o644
        });
        header.set_size(file_content.len() as u64);
        header.set_cksum();
        builder
            .append(&header, &file_content as &[u8])
            .or_else(|e| Err(format!("unable to append data file: {}", e)))?;
    }

    // We could also support maintainer scripts. For another day...

    builder
        .finish()
        .or_else(|e| Err(format!("unable to finish archive: {}", e)))?;

    Ok(())
}

pub fn execute_deb_archive(
    logger: &Logger,
    dist_path: &Path,
    control_paragraph: &debian::package::ControlParagraph,
    files: &FileManifest,
) -> Result<(), String> {
    let basename = format!(
        "{}_{}.deb",
        control_paragraph.get_entry("Package").unwrap(),
        control_paragraph.get_entry("Version").unwrap()
    );

    let dest_path = dist_path.join(basename);
    warn!(logger, "writing Debian package to {}", dest_path.display());

    let mut control_file = ControlFile::new();
    control_file.add_paragraph(control_paragraph.clone());

    let fh = std::fs::File::create(&dest_path)
        .or_else(|e| Err(format!("unable to create {}: {}", dest_path.display(), e)))?;

    build_deb(fh, &control_file, files)
}
