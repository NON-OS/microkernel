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

//! Ceilings on everything a program's output can make this crate hold.

/// Screen size bounds. A resize outside them is clamped.
pub const MIN_COLS: usize = 2;
pub const MIN_ROWS: usize = 1;
pub const MAX_COLS: usize = 1000;
pub const MAX_ROWS: usize = 500;

/// Scrollback lines kept when the host does not choose.
pub const DEFAULT_SCROLLBACK: usize = 5000;
pub const MAX_SCROLLBACK: usize = 100_000;

/// Numeric parameters in one control sequence, and the largest value one
/// may carry. A sequence with more parameters is ignored whole, because
/// acting on a truncated list would apply the wrong settings.
pub const MAX_PARAMS: usize = 32;
pub const MAX_PARAM: u16 = u16::MAX;
pub const MAX_INTERMEDIATES: usize = 2;

/// String sequences: OSC and DCS payloads past these are dropped whole.
pub const MAX_OSC: usize = 4096;
pub const MAX_DCS: usize = 1024;

pub const MAX_TITLE: usize = 256;
pub const MAX_TITLE_STACK: usize = 10;

/// Distinct OSC 8 hyperlink targets held, and the longest one kept.
pub const MAX_LINKS: usize = 1024;
pub const MAX_LINK: usize = 2048;

/// Reply bytes held for a program that is not reading them. Past this the
/// newest replies are dropped: a program that never reads cannot make the
/// terminal allocate without bound by asking questions.
pub const MAX_REPLY: usize = 4096;

/// Combining marks one line may carry.
pub const MAX_MARKS_PER_LINE: usize = 512;

/// Repeat count accepted by REP, and prompt starts remembered for jumping.
pub const MAX_REPEAT: usize = MAX_COLS * MAX_ROWS;
pub const MAX_PROMPT_MARKS: usize = 1024;
