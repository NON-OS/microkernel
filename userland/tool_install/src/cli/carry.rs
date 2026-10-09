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

//! What the command line carries into the new disk's store, read from the
//! running system through `std::fs`, which on NONOS is the vfs: a listing
//! returns every path under the prefix, a read the whole file.

use nonos_disk::CarrySource;

pub struct StdVfs;

impl CarrySource for StdVfs {
    fn list(&mut self, prefix: &str) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(prefix.trim_end_matches('/')) else {
            return Vec::new();
        };
        let path = |e: std::fs::DirEntry| {
            let p = e.path().to_string_lossy().into_owned();
            if e.file_type().is_ok_and(|t| t.is_dir()) {
                format!("{p}/")
            } else {
                p
            }
        };
        entries.filter_map(Result::ok).map(path).collect()
    }

    fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        std::fs::read(path).ok()
    }
}
