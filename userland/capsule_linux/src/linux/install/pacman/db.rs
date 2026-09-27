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

//! The repository databases: each fetched with its signature, verified,
//! then read for its `desc` records.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_openpgp::Key;

use super::desc::{records, Record};
use super::signed::signed;
use super::source::{repos, Source};
use super::unpacked::unpacked;
use crate::linux::install::tar::entries;

pub struct Db {
    /// Each record, with the repository it came from.
    pub records: Vec<(Record, &'static str)>,
}

/// An unsigned database could steer a dependency to some other file the
/// same key signed, an older one say; so a database without a verifying
/// signature refuses the whole install.
pub fn load(src: &Source, ring: &[Key]) -> Option<Db> {
    let mut db = Db { records: Vec::new() };
    for repo in repos() {
        let raw = src.get(repo, &format!("{repo}.db"))?;
        let Some(sig) = src.get(repo, &format!("{repo}.db.sig")) else {
            say(b"[LINUX] refused: the pacman database is not signed\n");
            return None;
        };
        if !signed(ring, &sig, &raw) {
            return None;
        }
        for e in entries(&unpacked(&raw)?) {
            if e.name.ends_with(b"/desc") {
                let text = String::from_utf8_lossy(&e.body);
                db.records.extend(records(&text).map(|r| (r, repo)));
            }
        }
    }
    (!db.records.is_empty()).then_some(db)
}

impl Db {
    /// A package by its own name first, then by a name it provides.
    pub fn find(&self, want: &str) -> Option<&(Record, &'static str)> {
        let named = self.records.iter().find(|(r, _)| r.name == want);
        named.or_else(|| self.records.iter().find(|(r, _)| r.provides.iter().any(|p| p == want)))
    }
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
