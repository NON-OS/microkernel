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

//! The folders a box leaves writable. A guest writes only its own private
//! tree (file/private/rule.rs); a folder named outside it would be a promise
//! the box cannot keep, so it is refused when the box is named.

use alloc::vec::Vec;

use super::preset::OWN;

/// `item` as a folder a box holds: absolute, inside one of the run's own
/// folders, no `.`, `..` or empty part, and no separator at its end.
pub fn folder(item: &[u8]) -> Result<Vec<u8>, &'static str> {
    let rest = item.strip_prefix(b"/").ok_or("a folder is named from /")?;
    if !rest.iter().all(|b| b.is_ascii_graphic()) {
        return Err("a folder is printable ASCII, with no space");
    }
    if rest.split(|b| *b == b'/').any(|p| p.is_empty() || p == b"." || p == b"..") {
        return Err("a folder has no empty, . or .. part");
    }
    if !OWN.iter().any(|own| under(item, own)) {
        return Err("a folder is inside /tmp, /home, /root, /run, /var/tmp or /dev/shm");
    }
    Ok(item.to_vec())
}

/// Whether `path`, as the guest sees it, is `folder` or below it.
pub fn under(path: &[u8], folder: &[u8]) -> bool {
    path.starts_with(folder) && matches!(path.get(folder.len()), None | Some(b'/'))
}

/// Whether a box with these folders lets `path` be written.
pub fn may_write(folders: &[Vec<u8>], path: &[u8]) -> bool {
    folders.iter().any(|f| under(path, f))
}
