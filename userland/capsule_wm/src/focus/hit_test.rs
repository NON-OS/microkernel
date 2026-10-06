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

use crate::window::{Kind, Visibility, Window, WindowTable};

pub struct HitTarget {
    pub owner_pid: u32,
    pub window_id: u32,
    pub local_x: u32,
    pub local_y: u32,
    pub win_x: u32,
    pub win_y: u32,
    pub win_w: u32,
    pub win_h: u32,
}

/// The window a press at (`px`, `py`) lands on, with the desktop shell's own
/// popup windows (its dock) above every other window. The shell draws its
/// chrome in a compositor band over every application window, so a window
/// dragged across the dock is drawn under it, and the click there must reach
/// the dock too. `chrome_pid` is the shell's pid, or 0 when it is not running;
/// no other process's popup gets the band, so an application cannot open one
/// over the dock to take its clicks.
pub fn topmost_hit_at(table: &WindowTable, px: u32, py: u32, chrome_pid: u32) -> Option<HitTarget> {
    let chrome = |w: &Window| chrome_pid != 0 && w.owner_pid == chrome_pid && w.kind == Kind::Popup;
    let rank = |w: &Window| (chrome(w), w.z);
    let mut best: Option<&Window> = None;
    for w in table.windows() {
        if w.visibility != Visibility::Visible {
            continue;
        }
        if !w.rect.contains(px, py) {
            continue;
        }
        if !w.kind.focusable() {
            continue;
        }
        best = Some(match best {
            None => w,
            Some(cur) if rank(w) > rank(cur) => w,
            Some(cur) => cur,
        });
    }
    best.map(|w| HitTarget {
        owner_pid: w.owner_pid,
        window_id: w.window_id,
        local_x: px.saturating_sub(w.rect.x),
        local_y: py.saturating_sub(w.rect.y),
        win_x: w.rect.x,
        win_y: w.rect.y,
        win_w: w.rect.width,
        win_h: w.rect.height,
    })
}
