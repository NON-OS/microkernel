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

//! Adding a group of files to the store whole or not at all: a program is
//! of no use without its certificate, manifest and trailer, and answers are
//! of no use without the marker that says they were kept.

use alloc::vec::Vec;

use nonos_disk_map::valid_name;

use super::builder::StoreBuilder;
use super::error::StoreError;

impl StoreBuilder {
    /// Check every file of `files` first, then add them all.
    pub fn add_all(&mut self, files: &[(&str, &[u8])]) -> Result<(), StoreError> {
        let names: Vec<&str> = files.iter().map(|(n, _)| *n).collect();
        if !names.iter().all(|n| valid_name(n)) {
            return Err(StoreError::BadName);
        }
        let seen = |n: &&str| self.names.iter().any(|m| m == n);
        let twice = names.iter().enumerate().any(|(i, n)| names[..i].contains(n));
        if names.iter().any(seen) || twice {
            return Err(StoreError::Duplicate);
        }
        let bytes = files.iter().map(|(_, d)| d.len() as u64).sum();
        if !self.has_room(files.len(), bytes) {
            return Err(StoreError::Full);
        }
        for (name, data) in files {
            self.add(name, data)?;
        }
        Ok(())
    }
}
