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

//! What an uninstall left undone, said in one line, so a REMOVE_PARTIAL
//! names the step that failed: the files and link lines that would not go,
//! and the program record, the manifest and the home when each did not.
//! Pure, for the host proofs.

use alloc::string::String;
use alloc::vec::Vec;

/// What stayed: counts of files and links not taken out, and whether each
/// of the last three steps was done.
pub struct Left {
    pub files: usize,
    pub links: usize,
    pub record: bool,
    pub manifest: bool,
    pub home: bool,
}

/// The line naming what stayed, or None when nothing did.
pub fn undone(pkg: &str, l: &Left) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if l.files > 0 {
        parts.push(alloc::format!("{} file(s)", l.files));
    }
    if l.links > 0 {
        parts.push(alloc::format!("{} link line(s)", l.links));
    }
    for (done, what) in
        [(l.record, "its program record"), (l.manifest, "its manifest"), (l.home, "its home")]
    {
        if !done {
            parts.push(String::from(what));
        }
    }
    if parts.is_empty() {
        return None;
    }
    Some(alloc::format!(
        "[LINUX] {pkg}: not taken out: {}\n",
        parts.join(", ")
    ))
}
