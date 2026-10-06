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

//! A Packages index: one stanza per package, `Field: value` lines with
//! continuation lines indented, a blank line between stanzas.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::hex::hex32;
use super::fields::{needs, safe};

#[derive(Default, Clone)]
pub struct Record {
    pub name: String,
    pub version: String,
    /// Relative to the mirror's root, as `pool/main/...`.
    pub filename: String,
    pub sha256: Option<[u8; 32]>,
    /// Each need is a group of alternatives, any one of which will do.
    pub depends: Vec<Vec<String>>,
    pub provides: Vec<String>,
}

/// Every installable stanza. One without a name, version, safe file name or
/// checksum is dropped rather than half-used.
pub fn stanzas(text: &str) -> Vec<Record> {
    let mut out = Vec::new();
    for stanza in text.split("\n\n") {
        let mut r = Record::default();
        // Continuation lines are skipped: no field read here spans lines.
        for line in stanza.lines().filter(|l| !l.starts_with([' ', '\t'])) {
            let Some((field, value)) = line.split_once(':') else {
                continue;
            };
            let value = value.trim();
            match field {
                "Package" => r.name = String::from(value),
                "Version" => r.version = String::from(value),
                "Filename" => r.filename = String::from(value),
                "SHA256" => r.sha256 = hex32(value),
                "Depends" | "Pre-Depends" => r.depends.extend(needs(value)),
                "Provides" => r.provides.extend(needs(value).into_iter().flatten()),
                _ => {}
            }
        }
        if !r.name.is_empty() && !r.version.is_empty() && safe(&r.filename) && r.sha256.is_some() {
            out.push(r);
        }
    }
    out
}
