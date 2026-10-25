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
use super::source::{repos, Source};
use crate::linux::install::pgp::signed;
use crate::linux::install::tar::entries;
use crate::linux::install::unpacked::unpacked;

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

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
