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

//! The path in the shell's answer to "is there a file for me to open". The
//! shell answers with a 4-byte status and then the path; a bare status means
//! it holds nothing for this app. Kept apart from the IPC so it can be proven.

/// `body` is the reply after the wire header.
pub fn reply_path(body: &[u8]) -> Option<&str> {
    match core::str::from_utf8(body.get(4..)?) {
        Ok(path) if path.starts_with('/') => Some(path),
        _ => None,
    }
}
