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

//! The folder in the shell's answer to "is there something for me to open".
//! The shell answers with a 4-byte status and then the path; a bare status
//! means it holds nothing for this app. The shell sends the file manager
//! folders only (a desktop folder icon), named without the trailing slash
//! the manager's listing prefixes carry, so one is added. Kept apart from the
//! IPC so it can be proven.

extern crate alloc;

use alloc::string::String;

/// `body` is the reply after the wire header.
pub fn reply_dir(body: &[u8]) -> Option<String> {
    let path = core::str::from_utf8(body.get(4..)?).ok()?;
    if !path.starts_with('/') {
        return None;
    }
    let mut dir = String::from(path);
    if !dir.ends_with('/') {
        dir.push('/');
    }
    Some(dir)
}
