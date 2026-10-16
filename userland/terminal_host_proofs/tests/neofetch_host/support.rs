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

//! Row text, escape stripping and info columns.

pub fn text(row: &[u8]) -> String {
    String::from_utf8(row.to_vec()).expect("a composed row is utf8")
}

pub fn strip(row: &[u8]) -> String {
    let mut out = Vec::new();
    let mut rest = row;
    while let Some(at) = rest.iter().position(|b| *b == 0x1b) {
        out.extend_from_slice(&rest[..at]);
        let end = rest[at..].iter().position(|b| *b == b'm').expect("an sgr escape ends in m");
        rest = &rest[at + end + 1..];
    }
    out.extend_from_slice(rest);
    text(&out)
}

pub fn info(lines: &[&str]) -> Vec<Vec<u8>> {
    lines.iter().map(|l| l.as_bytes().to_vec()).collect()
}
