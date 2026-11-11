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

use super::limits::{sidebar_full, sidebar_rail, toolbar_h, AUTO_FULL_MIN_W, MIN_CONTENT_W};
use super::rect::Rect;
use super::settings_cells::px;

/// How much room the sidebar takes. Auto picks Full or Rail by width; any
/// mode that would starve the page folds to a narrower one.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarMode {
    #[default]
    Auto,
    Full,
    Rail,
    Hidden,
}

impl SidebarMode {
    /// The mode the toolbar's sidebar button moves to: Full, Rail, Hidden, Full.
    pub fn cycled(self) -> SidebarMode {
        match self {
            SidebarMode::Auto | SidebarMode::Full => SidebarMode::Rail,
            SidebarMode::Rail => SidebarMode::Hidden,
            SidebarMode::Hidden => SidebarMode::Full,
        }
    }
}

/// Where the toolbar, sidebar and page content sit inside the window surface.
/// Every painter and every hit-test reads the same value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub toolbar: Rect,
    pub sidebar: Rect,
    pub content: Rect,
    pub rail: bool,
}

/// Split a `w` x `h` surface: toolbar across the top, sidebar down the left
/// below it, page content in the rest. Negative sizes count as zero.
pub fn compute(w: i32, h: i32, mode: SidebarMode) -> Layout {
    let w = w.max(0);
    let h = h.max(0);
    let bar = toolbar_h().min(h);
    let side = sidebar_w(w, mode);
    let below = h - bar;
    Layout {
        toolbar: Rect::new(0, 0, w, bar),
        sidebar: Rect::new(0, bar, side, below),
        content: Rect::new(side, bar, w - side, below),
        rail: side == sidebar_rail(),
    }
}

fn sidebar_w(w: i32, mode: SidebarMode) -> i32 {
    let (full, rail) = (sidebar_full(), sidebar_rail());
    let wanted = match mode {
        SidebarMode::Hidden => 0,
        SidebarMode::Rail => rail,
        SidebarMode::Full => full,
        SidebarMode::Auto if w >= px(AUTO_FULL_MIN_W as u32) => full,
        SidebarMode::Auto => rail,
    };
    let room = px(MIN_CONTENT_W as u32);
    [wanted, rail, 0]
        .into_iter()
        .filter(|&s| s <= wanted)
        .find(|&s| s == 0 || w - s >= room)
        .unwrap_or(0)
}
