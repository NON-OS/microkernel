// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A repository database's `desc` records: `%FIELD%` on a line, its values
//! on the lines after, a blank line ending it.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::hex::hex32;

#[derive(Default, Clone)]
pub struct Record {
    pub name: String,
    pub version: String,
    pub filename: String,
    pub sha256: Option<[u8; 32]>,
    /// The detached signature, base64 as the database writes it.
    pub pgpsig: String,
    /// Names needed, version constraints dropped.
    pub depends: Vec<String>,
    pub provides: Vec<String>,
}

/// One `desc` file. A record without a name, version, file or checksum is not
/// installable and is dropped rather than half-used.
pub fn records(text: &str) -> Option<Record> {
    let mut r = Record::default();
    let mut field = "";
    for line in text.lines() {
        if let Some(f) = line.strip_prefix('%').and_then(|l| l.strip_suffix('%')) {
            field = f;
            continue;
        }
        if line.is_empty() {
            field = "";
            continue;
        }
        match field {
            "NAME" => r.name = String::from(line),
            "VERSION" => r.version = String::from(line),
            "FILENAME" => r.filename = String::from(line),
            "SHA256SUM" => r.sha256 = hex32(line),
            "PGPSIG" => r.pgpsig.push_str(line),
            "DEPENDS" => r.depends.push(bare(line)),
            "PROVIDES" => r.provides.push(bare(line)),
            _ => {}
        }
    }
    let whole = !r.name.is_empty() && !r.version.is_empty() && !r.filename.is_empty();
    // A file name that could leave the repository directory is refused.
    let safe = !r.filename.contains('/') && r.filename != ".." && r.filename != ".";
    (whole && safe && r.sha256.is_some()).then_some(r)
}

/// `glibc>=2.35` and `libfoo.so=1-64` name `glibc` and `libfoo.so`.
fn bare(dep: &str) -> String {
    String::from(dep.split(['<', '>', '=']).next().unwrap_or(dep))
}
