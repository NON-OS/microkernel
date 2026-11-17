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

//! Which control of the titlebar strip lies under a point, read from the
//! same boxes the strip is drawn in.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Tabs;

use super::strip::{close, plus, tab_area, tools};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spot {
    Tab(usize),
    Close(usize),
    Plus,
    Tool(usize),
}

/// What a pointer event on the strip comes to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step {
    Idle,
    Repaint,
    Act(Spot),
}

/// The control under `(x, y)`. A tab's close mark answers only while there
/// is another tab to fall back to.
pub fn spot(labels: &[&str], active: usize, (w, h): (u32, u32), x: i32, y: i32) -> Option<Spot> {
    if let Some(i) = tools(w, h).iter().position(|&r| contains(r, x, y)) {
        return Some(Spot::Tool(i));
    }
    let tabs = Tabs::new(labels, active);
    if contains(plus(&tabs, w, h, labels.len()), x, y) {
        return Some(Spot::Plus);
    }
    let i = tabs.hit(tab_area(w, h), x, y)?;
    let body = tabs.tab(tab_area(w, h), i)?;
    let closes = labels.len() > 1 && close(body).is_some_and(|c| contains(c, x, y));
    Some(if closes { Spot::Close(i) } else { Spot::Tab(i) })
}
