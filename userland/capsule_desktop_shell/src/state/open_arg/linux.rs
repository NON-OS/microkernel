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

//! The command line a package run hands the Terminal: `linux <package>`, run
//! as typed. The shell writes the line itself from a name that can only be a
//! package name, so the one asking chooses which package, never what runs.

use alloc::string::String;

use super::command::RUN;

/// The longest package name taken. Alpine's longest are well under it.
pub const NAME_MAX: usize = 64;

/// Whether `name` is a package name: a lower-case letter or digit first,
/// then those, `.`, `_`, `+` or `-`. Nothing a shell reads as a word break,
/// a quote, a redirection or a path.
pub fn is_package(name: &[u8]) -> bool {
    let first = name.first().is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    let rest = name.iter().all(|b| {
        b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'+' | b'-')
    });
    first && rest && name.len() <= NAME_MAX
}

/// `run:linux <name>` for the Terminal, or None when `name` is not a
/// package name.
pub fn linux_command(name: &[u8]) -> Option<String> {
    if !is_package(name) {
        return None;
    }
    let name = core::str::from_utf8(name).ok()?;
    let mut line = String::with_capacity(RUN.len() + 6 + name.len());
    line.push_str(RUN);
    line.push_str("linux ");
    line.push_str(name);
    Some(line)
}
