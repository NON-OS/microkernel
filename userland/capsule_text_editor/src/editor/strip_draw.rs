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

//! Hitting, lighting and drawing the tab strip, each from `strip`'s boxes.

use alloc::vec::Vec;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{IconButton, Pointer, Rect, State, Surface, Tabs};

use super::strip::{Spot, Strip, TOOLS};

fn within(at: u32, len: u32, p: i32) -> bool {
    (i64::from(at)..i64::from(at) + i64::from(len)).contains(&i64::from(p))
}

impl Strip<'_> {
    /// The control whose column, within the band, holds the point.
    pub fn spot(&self, x: i32, y: i32) -> Option<Spot> {
        within(self.band.y, self.band.h, y).then_some(())?;
        let across = |r: Rect| within(r.x, r.w, x);
        if let Some(t) = self.tools().into_iter().position(across) {
            return Some(Spot::Tool(t));
        }
        let i = (0..self.names.len()).find(|&i| self.tab(i).is_some_and(across))?;
        Some(if self.close(i).is_some_and(across) { Spot::Close(i) } else { Spot::Tab(i) })
    }

    /// The hover and press `spot` draws with.
    pub fn look(&self, ptr: &Pointer, spot: Spot) -> State {
        ptr.state(|x, y| self.spot(x, y) == Some(spot))
    }

    /// The tab the pointer is on, its close mark included.
    pub fn hovered(&self, ptr: &Pointer) -> Option<usize> {
        match ptr.position().and_then(|(x, y)| self.spot(x, y)) {
            Some(Spot::Tab(i) | Spot::Close(i)) => Some(i),
            _ => None,
        }
    }

    /// The hovered tab, then the look of every close mark and view control.
    pub fn lit(&self, ptr: &Pointer) -> (Option<usize>, Vec<State>) {
        let closes = (0..self.names.len()).map(Spot::Close);
        let spots = closes.chain((0..TOOLS.len()).map(Spot::Tool));
        (self.hovered(ptr), spots.map(|s| self.look(ptr, s)).collect())
    }

    /// Paints the tabs, each one's close or unsaved mark, and the controls.
    pub fn draw(&self, fb: &mut impl Surface, ptr: &Pointer) {
        Tabs::new(self.names, self.active).hovered(self.hovered(ptr)).draw(fb, self.area());
        for i in 0..self.names.len() {
            let Some(mark) = self.close(i) else { continue };
            let icon =
                if self.dirty.get(i) == Some(&true) { IconId::Modified } else { IconId::Close };
            IconButton::new(icon).state(self.look(ptr, Spot::Close(i))).draw(fb, mark);
        }
        for (t, (r, icon)) in self.tools().into_iter().zip(TOOLS).enumerate() {
            IconButton::new(icon).state(self.look(ptr, Spot::Tool(t))).draw(fb, r);
        }
    }
}
