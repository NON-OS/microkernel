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

//! What the installer's and the file service's refusals say, and the
//! Launchpad's empty search.

/// What the installer's answer to a package query or commit means, in the
/// words the Terminal's `pkg` uses, short enough for a toast after
/// "Package: ". Values are the installer's errnos.
pub fn package(code: i32) -> &'static [u8] {
    match code {
        -2 => b"not found",
        -5 => b"store write failed",
        -11 => b"installer not ready, try again",
        -13 => b"signature or digest failed verification",
        -17 => b"already installed",
        -22 => b"malformed package or path",
        -71 => b"malformed reply from installer",
        _ => b"the installer refused it",
    }
}

/// Why the file service refused a desktop action. Values are vfs_pool's
/// errnos; `NO_REPLY` is the shell's own for a call nothing answered.
pub const NO_REPLY: i32 = i32::MIN;

pub fn vfs(code: i32) -> &'static [u8] {
    match code {
        NO_REPLY => b"the file service did not answer",
        -2 => b"it is not there",
        -13 => b"access denied",
        -17 => b"that name is taken",
        -22 => b"not a valid name",
        -28 => b"no space left",
        -39 => b"the folder is not empty",
        _ => b"the file service refused",
    }
}

/// What the Launchpad shows when its search leaves no app, tool or package
/// standing: that nothing matched, and the two ways out. None while
/// something matches, or before anything is typed.
pub fn launchpad_empty(query: &str, matched: usize) -> Option<[&'static str; 2]> {
    if matched != 0 || query.is_empty() {
        return None;
    }
    Some([
        "No app, tool or package has that in its name.",
        "Backspace to change it, Esc to clear it.",
    ])
}
