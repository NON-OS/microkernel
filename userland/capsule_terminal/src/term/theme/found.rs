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

//! The ground under a search match. It leans from the theme's ground towards
//! its quiet ink, as far as it can while the text on it still clears the body
//! floor, so a theme with little contrast to spare takes a fainter wash.

use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::derive::{mix, opaque};

use super::types::Theme;

/// How far, of 255, a match leans towards the quiet ink at most, and the
/// step it backs off by while the text on it would not read.
const REACH: u8 = 150;
const STEP: usize = 6;

pub fn found(t: &Theme) -> u32 {
    let lean = |s: u8| opaque(mix(t.bg, t.run, s));
    let mut steps = (0..=REACH).rev().step_by(STEP).map(lean);
    steps.find(|&c| at_least(t.fg, c, BODY)).unwrap_or(opaque(t.bg))
}
