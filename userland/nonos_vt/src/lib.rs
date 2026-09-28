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

//! Terminal emulation: the bytes a program writes become a screen of cells,
//! and the keys a person presses become the bytes that program reads.
//!
//! The parser follows the DEC state machine, so every byte has one meaning in
//! every state and nothing a program writes can leave it stuck. The screen
//! answers what xterm answers for the sequences full-screen tools depend on,
//! resizes by re-wrapping its lines, and keeps a bounded scrollback. Every
//! buffer that hostile output could grow has a fixed ceiling in `limits`.

#![no_std]

extern crate alloc;

pub mod cell;
pub mod charset;
pub mod color;
pub mod input;
pub mod limits;
pub mod line;
mod out;
pub mod params;
pub mod parser;
pub mod term;
pub mod url;
pub mod utf8;
pub mod width;

pub use cell::{Cell, Pen, Underline};
pub use color::{Color, Palette};
pub use line::Line;
pub use term::{ClipboardRequest, CursorShape, CursorView, Modes, MouseMode, Pos, Term};
