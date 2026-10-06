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

//! The folders the catalogue reads, and the names the Folders page gives
//! them. The scan and the page share this one list, so the page can only
//! offer a folder that was actually read.

use super::entry::parent_dir;

pub const ROOTS: [&str; 5] = ["/", "/Movies", "/Series", "/Downloads", "/Clips"];

pub const LABELS: [&str; ROOTS.len()] = ["Top level", "Movies", "Series", "Downloads", "Clips"];

/// Whether the video at `path` sits directly in root `folder`.
pub fn in_folder(path: &str, folder: usize) -> bool {
    ROOTS.get(folder).is_some_and(|root| parent_dir(path) == *root)
}
