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

use super::header_slots::strip;
use super::info_geom::{info_geom, InfoGeom};
use super::info_perms::perm_btn_w;
use super::info_quick::quick_slots;
use super::info_sizes::QUICK_H;
use super::info_tag_hit::tag_aim;
use super::layout::{info_x, px};
use super::screen::Screen;
use super::state::State;
use super::target::Target;

/// The info strip owns its whole column. Returning `Some` for any point inside
/// it, `Some(None)` where no control is, is what stops a click on the panel
/// from falling through to the row hit-test, which resolves on `y` alone and
/// would select an unrelated row.
pub fn info_at(state: &State, x: u32, y: u32) -> Option<Option<Target>> {
    if state.screen != Screen::Browse || x < info_x(state.win_w) {
        return None;
    }
    let Some(g) = info_geom(state, state.win_w, state.win_h) else {
        return Some(None);
    };
    Some(control(state, &g, x, y))
}

fn control(state: &State, g: &InfoGeom, x: u32, y: u32) -> Option<Target> {
    let quick_h = px(QUICK_H);
    if y >= g.quick_y && y < g.quick_y + quick_h && g.quick_y + quick_h <= g.bottom {
        if let Some(i) = quick_slots(g.x, g.w).iter().position(|s| x >= s.x && x < s.x + s.w) {
            return Some(Target::Quick(i));
        }
    }
    if let Some(t) = tag_aim(state, g, x, y) {
        return Some(t);
    }
    perms_hit(state, g, x, y).then_some(Target::Perms)
}

fn perms_hit(state: &State, g: &InfoGeom, x: u32, y: u32) -> bool {
    let Some(entry) = state.entries.get(state.cursor) else { return false };
    let w = perm_btn_w(entry.writable).min(g.w);
    let tool_h = strip().h;
    g.perms_btn + tool_h <= g.bottom
        && y >= g.perms_btn
        && y < g.perms_btn + tool_h
        && x >= g.x
        && x < g.x + w
}
