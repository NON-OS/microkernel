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

//! `version`: the NONOS release the session runs on, read from the tree's
//! VERSION file at build time, as the terminal's `version` reads it.

use crate::out::ShellOut;

const VERSION: &str = include_str!("../../../../VERSION");

/// The line `version` prints: "NONOS " and the release.
pub fn release_line() -> alloc::vec::Vec<u8> {
    let mut line = alloc::vec::Vec::from(&b"NONOS "[..]);
    line.extend_from_slice(VERSION.trim_end().as_bytes());
    line
}

pub(super) fn version(out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    if w.len() > 1 {
        out.writeln(b"usage: version");
        return false;
    }
    out.writeln(&release_line());
    true
}
