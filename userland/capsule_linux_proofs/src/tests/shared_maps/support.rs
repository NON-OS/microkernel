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

//! Maps made the way the tests name them: by pages.

pub use crate::linux::file::shared_maps::entry::Entry;

pub const P: u64 = crate::linux::guest::PAGE;

/// A read-only map of `path` by `pid`: `pages` at page `at`, from file page `off`.
pub fn map(pid: u32, at: u64, pages: u64, path: &str, off: u64) -> Entry {
    let path = path.as_bytes().to_vec();
    let (at, span, off) = (at * P, pages * P, off * P);
    Entry { pid, at, span, path, off, writer: false, may_write: false, orphan: false }
}

/// The same map through a descriptor open to write.
pub fn may(e: Entry) -> Entry {
    Entry { may_write: true, ..e }
}

/// The same map, writable.
pub fn writer(e: Entry) -> Entry {
    Entry { writer: true, may_write: true, ..e }
}
