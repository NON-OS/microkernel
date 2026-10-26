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

//! A whole kernel line begins at column 0 even after a capsule's MkDebug
//! text that has not yet sent its newline.

use super::column::{note, opening};

const PRINT: &str = include_str!("../../../../src/sys/serial/print.rs");

#[test]
fn a_whole_line_starts_at_column_zero_after_a_strangers_partial_one() {
    b"[BLK] step claim".iter().for_each(|&b| note(b));
    assert_eq!(opening(), b"\r\n", "the partial line is ended first");
    b"\r\n".iter().for_each(|&b| note(b));
    assert_eq!(opening(), b"", "at a line's start nothing is added");
    let line = &PRINT[PRINT.find("pub fn line(").unwrap()..];
    let flat: String = line[..line.find("\n}").unwrap()].split_whitespace().collect();
    assert!(flat.contains("emit(super::column::opening());emit(s);"), "opened, then the line");
}
