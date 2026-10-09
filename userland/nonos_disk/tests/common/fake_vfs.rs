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

//! A running system's vfs in memory, as the carry sees one: whole paths,
//! directories with a trailing slash, and a listing that returns anything
//! merely starting with the prefix, as `std::fs::read_dir` on NONOS does.
//! Files named in `unreadable` are listed and never returned.

use std::collections::{BTreeMap, BTreeSet};

use nonos_disk::CarrySource;

#[derive(Default)]
pub struct FakeVfs {
    pub files: BTreeMap<String, Vec<u8>>,
    pub unreadable: BTreeSet<String>,
}

impl FakeVfs {
    pub fn put(&mut self, path: &str, data: &[u8]) {
        self.files.insert(path.to_string(), data.to_vec());
    }
}

impl CarrySource for FakeVfs {
    fn list(&mut self, prefix: &str) -> Vec<String> {
        let loose = prefix.trim_end_matches('/');
        let mut out: Vec<String> =
            self.files.keys().filter(|p| p.starts_with(loose)).cloned().collect();
        out.push(format!("{loose}/some/dir/"));
        out
    }

    fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        if self.unreadable.contains(path) {
            return None;
        }
        self.files.get(path).cloned()
    }
}
