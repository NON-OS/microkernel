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

use alloc::string::String;

/* The operator of an attribute selector: presence, or one of the six value
 * tests (=, *=, ^=, $=, ~=, |=). */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttrOp {
    Present,
    Eq,
    Contains,
    Starts,
    Ends,
    Word,
    Lang,
}

/* One attribute test. The `i` flag asks for an ASCII case-insensitive
 * comparison of the value; without it the value compares exactly. */
#[derive(Clone)]
pub struct AttrTest {
    pub op: AttrOp,
    pub value: String,
    pub case_insensitive: bool,
}

/* CSS whitespace: space, tab, line feed, carriage return, form feed. */
pub fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0c)
}
