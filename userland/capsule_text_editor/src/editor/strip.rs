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

//! Where the document tab strip puts each control: the tabs from the band's
//! left, a close mark in each tab's trailing padding, and the view controls
//! flush right. The painter, the pointer and the click all read these boxes.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Rect, Tabs};

/// The view controls, left to right: zoom out, zoom in, the next theme.
pub const TOOLS: [IconId; 3] = [IconId::ZoomOut, IconId::ZoomIn, IconId::Theme];

/// The control a point in the band falls on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spot {
    Tab(usize),
    Close(usize),
    Tool(usize),
}

/// One frame's strip: a label and an unsaved flag per document, the chosen
/// one, and the band it fills.
#[derive(Clone, Copy)]
pub struct Strip<'a> {
    pub names: &'a [&'a str],
    pub dirty: &'a [bool],
    pub active: usize,
    pub band: Rect,
}

impl Strip<'_> {
    /// The view controls, each a square the band's height, flush right.
    pub fn tools(&self) -> [Rect; 3] {
        let (b, h) = (self.band, self.band.h);
        let base = b.x.saturating_add(b.w).saturating_sub(h.saturating_mul(3));
        core::array::from_fn(|i| Rect { x: base + i as u32 * h, w: h, ..b })
    }

    /// The box the tabs are laid in, short of the view controls.
    pub fn area(&self) -> Rect {
        let room = self.band.h.saturating_mul(3).saturating_add(scale().px(u32::from(SPACE.xs)));
        Rect { w: self.band.w.saturating_sub(room), ..self.band }
    }

    /// The body of tab `i`, as `Tabs` lays it out.
    pub fn tab(&self, i: usize) -> Option<Rect> {
        Tabs::new(self.names, self.active).tab(self.area(), i)
    }

    /// The close mark of tab `i`: a square in the padding after its label. A
    /// tab squeezed narrower than its two paddings has none.
    pub fn close(&self, i: usize) -> Option<Rect> {
        let (body, side) = (self.tab(i)?, scale().px(u32::from(SPACE.md)));
        let x = body.x.checked_add(body.w.checked_sub(side.saturating_mul(2))?)? + side;
        Some(Rect { x, y: body.y + body.h.saturating_sub(side) / 2, w: side, h: side })
    }
}
