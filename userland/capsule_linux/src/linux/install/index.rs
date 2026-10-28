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

//! The distribution's package index, as this capsule needs it.

use super::pkg::bare;
pub use super::pkg::Pkg;
use alloc::string::String;
use alloc::vec::Vec;

#[path = "index_installed.rs"]
mod installed;
/// Each soname, and each name or provided name, to the package that has it.
/// The lookups are in index_find.rs, and index_installed.rs.
#[derive(Default)]
pub struct Index {
    pub(super) pkgs: Vec<Pkg>,
    pub(super) libs: Vec<(String, usize)>,
    pub(super) names: Vec<(String, usize)>,
}

impl Index {
    /// Records end at a blank line; `D:` and `p:` follow `V:`. Each is
    /// marked as listed by `branch`.
    pub fn add(&mut self, raw: &[u8], branch: usize) {
        let text = String::from_utf8_lossy(raw);
        let (mut cur, mut provides) = (Pkg::default(), Vec::new());
        for line in text.lines().chain(core::iter::once("")) {
            let rest = line.get(2..).unwrap_or("");
            match line.as_bytes().first() {
                None => {
                    cur.branch = branch;
                    self.finish(core::mem::take(&mut cur), core::mem::take(&mut provides))
                }
                Some(b'C') => cur.checksum = super::auth::checksum(rest),
                Some(b'P') => cur.name = String::from(rest),
                Some(b'S') => cur.size = rest.parse().unwrap_or(0),
                Some(b'I') => cur.installed = rest.parse().unwrap_or(0),
                Some(b'V') => cur.version = String::from(rest),
                Some(b'D') => cur.read_depends(rest),
                Some(b'i') => cur.read_install_if(rest),
                Some(b'p') => provides = rest.split_whitespace().map(bare).collect(),
                _ => {}
            }
        }
    }

    fn finish(&mut self, pkg: Pkg, provides: Vec<String>) {
        if pkg.name.is_empty() || pkg.version.is_empty() {
            return;
        }
        let at = self.pkgs.len();
        self.names.push((pkg.name.clone(), at));
        for name in provides {
            match name.strip_prefix("so:") {
                Some(so) => self.libs.push((String::from(so), at)),
                None if !name.contains(':') => self.names.push((name, at)),
                None => {}
            }
        }
        self.pkgs.push(pkg);
    }
}
