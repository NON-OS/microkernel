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

//! The directory the desktop shows, and the path of a name on it. Every
//! desktop action (list, open, new, rename, move, delete) builds its path
//! here: New Folder, New File, Rename and Delete once acted on `/` while the
//! icons were the listing of the home directory, so a new folder never
//! appeared and Delete removed a system entry of the same name, if any.

use alloc::string::String;

/// Where the desktop looks. One definition, so the listing and anything that
/// later resolves an icon to a path cannot disagree about which directory the
/// desktop is showing.
pub const HOME: &[u8] = b"/home/nonos";

/// The full path of `name` on the desktop, or None for a name that is not a
/// single entry of it: empty, "." or "..", or holding a '/' or a NUL.
pub fn home_path(name: &str) -> Option<String> {
    if !is_entry_name(name) {
        return None;
    }
    let home = core::str::from_utf8(HOME).ok()?;
    let mut path = String::with_capacity(home.len() + 1 + name.len());
    path.push_str(home);
    path.push('/');
    path.push_str(name);
    Some(path)
}

/// Whether `name` names one entry directly in the desktop's directory.
pub fn is_entry_name(name: &str) -> bool {
    !name.is_empty() && name != "." && name != ".." && !name.contains(['/', '\0'])
}
