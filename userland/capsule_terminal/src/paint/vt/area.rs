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

//! Where the body is drawn, with what, and what is shaded over it.

use nonos_vt::{Pos, Term};

use crate::paint::metrics::Metrics;
use crate::term::select::Selection;
use crate::term::theme::types::Theme;

pub const OPAQUE: u32 = 0xFF00_0000;

/// What is shaded over the text.
#[derive(Clone, Copy, Default)]
pub struct Shade {
    pub selection: Option<Selection>,
    pub found: Option<(Pos, Pos)>,
}

/// The rectangle the body may draw in.
pub struct Area {
    pub x: u32,
    pub y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

/// One frame's drawing context.
pub struct Frame<'a> {
    pub vt: &'a Term,
    pub area: Area,
    pub m: Metrics,
    pub t: &'a Theme,
}
