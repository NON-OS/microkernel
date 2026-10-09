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

//! What the menu shows: the selection, the countdown, and what to repaint.

use crate::security::SecurityContext;

/// What changed since the last frame, so a key press repaints only the card
/// and the text under it, and a tick of the countdown only the footer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Dirty {
    All,
    Selection,
    Timer,
}

pub(super) struct Frame<'a> {
    pub sel: usize,
    pub default: usize,
    pub remaining_s: u32,
    pub total_s: u32,
    /// The first frame leaves the emblem to the opening animation.
    pub intro: bool,
    pub sec: &'a SecurityContext,
}

impl<'a> Frame<'a> {
    pub fn new(default: usize, total_s: u32, sec: &'a SecurityContext) -> Self {
        Frame { sel: default, default, remaining_s: total_s, total_s, intro: true, sec }
    }
}
