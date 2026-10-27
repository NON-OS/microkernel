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

//! Fetching and joining the branch indexes.

use alloc::format;

use super::super::auth::{rsa_verify, signed_index};
use super::super::http::get::get;
use super::super::index::Index;
use super::super::run::{ARCH, BRANCHES, RELEASE};
use super::super::tar::entries;

pub(in crate::linux::install) fn load_index() -> Option<Index> {
    let mut index = Index::default();
    for (at, branch) in BRANCHES.iter().enumerate() {
        let path = format!("/alpine/{RELEASE}/{branch}/{ARCH}/APKINDEX.tar.gz");
        let raw = get(&path)?;
        /*
         * A branch whose signature does not verify refuses the whole index:
         * resolving against half of it would pick a dependency from whichever
         * branch happened to be authentic.
         */
        let Some(plain) = signed_index(&raw, &rsa_verify) else {
            say(b"[LINUX] package index signature did not verify\n");
            return None;
        };
        for entry in entries(&plain) {
            if entry.name.ends_with(b"APKINDEX") {
                index.add(&entry.body, at);
            }
        }
    }
    Some(index)
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
