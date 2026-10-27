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

//! The Debian packages a NONOS-mirror program runs on, pinned in its own
//! content (tools/nonos_market_catalogue/asset_needs.py reads the same file).
//! One line a package: `<name> <version> <pool file> <sha256>`. The content
//! is held to the market's BLAKE3 pin, so each SHA-256 here is the signed
//! index's word, not the mirror's. Pure, so capsule_linux_proofs holds it.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::hex::hex32;
use super::fields::safe;
use super::packages::Record;

/// Where in a tool's content its pins are, under the tool's own name.
pub const NEEDS_DIR: &str = "usr/share/nonos-debs/";
/// More pins than any closure needs is a file gone wrong, and refused.
pub const MAX_NEEDS: usize = 256;

pub fn needs_path(tool: &str) -> String {
    alloc::format!("{NEEDS_DIR}{tool}")
}

/// Each line as a record with no needs of its own, or None when a line is
/// not four fields of the right shape, or a name comes twice: the whole
/// file or nothing, so no closure is installed from a file half read.
pub fn pinned(text: &str) -> Option<Vec<Record>> {
    let mut out: Vec<Record> = Vec::new();
    for line in text.lines() {
        let mut f = line.split(' ');
        let (name, version, filename, sum) = (f.next()?, f.next()?, f.next()?, f.next()?);
        if f.next().is_some() || !debian_name(name) || !debian_version(version) || !safe(filename) {
            return None;
        }
        if out.len() == MAX_NEEDS || out.iter().any(|r| r.name == name) {
            return None;
        }
        let (name, version, filename) = (name.into(), version.into(), filename.into());
        let sha256 = Some(hex32(sum)?);
        out.push(Record { name, version, filename, sha256, ..Record::default() });
    }
    (!out.is_empty()).then_some(out)
}

/// Debian policy 5.6.1: two or more of a-z, 0-9, `+`, `-`, `.`, from an
/// alphanumeric.
fn debian_name(n: &str) -> bool {
    let fits = |c: u8| c.is_ascii_lowercase() || c.is_ascii_digit() || b"+-.".contains(&c);
    let first = n.bytes().next().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    (2..=64).contains(&n.len()) && first && n.bytes().all(fits)
}

fn debian_version(v: &str) -> bool {
    (1..=128).contains(&v.len()) && v.bytes().all(|c| c.is_ascii_graphic())
}
