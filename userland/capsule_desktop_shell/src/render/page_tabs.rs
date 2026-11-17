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
//! The Launchpad's page switcher as shared `Tabs`, one numbered tab a page,
//! centred on the band under the grid and lit by the shell's `Pointer`. The
//! paint and the click share the box, so a tab selects the page it shows.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use nonos_toolkit::widgets::{Pointer, Rect, Surface, Tabs};

/// One label a page, numbered from one.
pub fn labels(pages: usize) -> Vec<String> {
    (1..=pages).map(|n| n.to_string()).collect()
}

/// The strip's box: its natural width, no wider than the screen, centred
/// across `width` on the `band` above `foot`.
pub fn place(tabs: &Tabs, width: u32, foot: u32, band: u32) -> Rect {
    let w = tabs.size().0.min(width);
    Rect { x: (width - w) / 2, y: foot.saturating_sub(band), w, h: band }
}

/// The tab under the pointer, unless it is pressed off it.
pub fn lit(tabs: &Tabs, at: Rect, pointer: &Pointer) -> Option<usize> {
    let i = pointer.position().and_then(|(x, y)| tabs.hit(at, x, y))?;
    pointer.state(|x, y| tabs.hit(at, x, y) == Some(i)).hovered.then_some(i)
}

pub fn draw(fb: &mut impl Surface, tabs: Tabs, at: Rect, pointer: &Pointer) {
    tabs.hovered(lit(&tabs, at, pointer)).draw(fb, at);
}
