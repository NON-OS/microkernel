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

//! What the manager actually knows about permissions is one bit: `stat_full`
//! reports writable or not, and only for files -- `refresh_meta` skips
//! directories, so a directory's bit is the `true` it was built with. There is
//! no owner, group or mode in the reply, so this block states the bit and the
//! octal mode the toggle will write, and claims nothing else.

extern crate alloc;

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Button, Rect};

use super::entries::Entry;
use super::header_slots::strip;
use super::info_geom::InfoGeom;
use super::info_sizes::SEC_PX;
use super::layout::font;
use super::perms::{MODE_RO, MODE_RW};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const HEADING: &str = "Permissions";

/// The label the toggle carries, which names both the direction and the mode it
/// will chmod to, because the store reports no mode of its own.
pub fn perm_label(writable: bool) -> String {
    let (verb, mode) =
        if writable { ("read-only", MODE_RO) } else { ("writable", MODE_RW) };
    alloc::format!("Make {verb} ({mode:04o})")
}

/// The button's width, the shared `Button`'s own, so the hit-test matches.
pub fn perm_btn_w(writable: bool) -> u32 {
    Button::new(perm_label(writable).as_str()).size().0
}

/// Drawn only when the panel has room, and the hit-test applies the same clip.
pub fn paint_perms(state: &State, fb: &mut PaintBuffer, g: &InfoGeom, entry: &Entry) {
    let tool_h = strip().h;
    if g.perms_btn + tool_h > g.bottom {
        return;
    }
    let muted = roles().muted.argb();
    let _ = fb.text_ttf(g.x as i32, g.perms_head as i32, HEADING, muted, font(SEC_PX));
    let w = perm_btn_w(entry.writable).min(g.w);
    let label = perm_label(entry.writable);
    let at = Rect { x: g.x, y: g.perms_btn, w, h: tool_h };
    Button::new(label.as_str()).state(look(state, Target::Perms)).draw(fb, at);
}
