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

//! The terminal's modes and the small types its public surface uses.

use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CursorShape {
    #[default]
    Block,
    Underline,
    Bar,
}

/// Where and how the cursor is drawn on the visible rows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CursorView {
    pub x: usize,
    pub y: usize,
    /// False when the program hid it or the view is scrolled away from it.
    pub visible: bool,
    pub shape: CursorShape,
    pub blink: bool,
}

/// A position in the whole history: an absolute line number, counted from
/// the first line this terminal ever had, and a column.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Pos {
    pub line: u64,
    pub col: usize,
}

/// OSC 52: a program asked to set a clipboard. The host decides whether to
/// honour it. Asking to read a clipboard is never answered.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClipboardRequest {
    /// The selection names the program gave, such as `c` or `p`.
    pub targets: Vec<u8>,
    /// The text, still base64 encoded as the program sent it.
    pub base64: Vec<u8>,
}
