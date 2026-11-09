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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::Controls;

use crate::browser::paint::sidebar::{brand, nav};
use crate::browser::paint::tokens::{self, palette};
use crate::browser::shell::cards::members;
use crate::browser::shell::groups::{count, MAX_GROUPS};
use crate::browser::shell::Shell;
use crate::browser::ui::group_rows::dot_slot;
use crate::browser::ui::layout::Layout;
use crate::browser::ui::sidebar_geom::{self, RowKind};

const DIVIDER_PAD: i32 = 12;

/// The sidebar: its fill and right edge, the brand row in full mode, the
/// two divider hairlines, then every row laid out in `c` and drawn, the
/// row standing for what the window shows lit (`on`), and each group's
/// colour dot in the slot before its name.
pub fn paint(
    shell: &Shell,
    on: Option<RowKind>,
    lay: &Layout,
    fb: &mut PaintBuffer,
    c: &mut Controls<RowKind>,
) {
    let s = lay.sidebar;
    if s.w <= 0 || s.h <= 0 {
        c.clear();
        return;
    }
    fb.fill_rect(s.x as u32, s.y as u32, s.w as u32, s.h as u32, palette().sidebar);
    fb.fill_rect((s.right() - 1) as u32, s.y as u32, 1, s.h as u32, palette().edge);
    brand::paint(sidebar_geom::brand(lay.rail, s), fb);
    let dw = (s.w - 2 * DIVIDER_PAD - 1).max(0) as u32;
    let n = shell.groups.len();
    for y in sidebar_geom::dividers(lay.rail, s, n) {
        fb.fill_rect((s.x + DIVIDER_PAD) as u32, y.max(0) as u32, dw, 1, palette().edge);
    }
    let held = members(shell);
    let groups: Vec<(&str, usize)> =
        shell.groups.iter().map(|g| (g.name.as_str(), count(g.id, &held))).collect();
    let k = nav::Look { on, tabs: held.len(), groups: &groups, full: n >= MAX_GROUPS };
    for r in nav::paint(fb, lay.rail, s, &k, c) {
        let RowKind::Group(i) = r.kind else { continue };
        let (Some(g), d) = (shell.groups.get(i), dot_slot(r.rect).0) else { continue };
        let tint = tokens::category(Category::new(g.color as usize)).0;
        let (x, y, w) = (d.x as u32, d.y as u32, d.w as u32);
        fb.fill_round(x, y, w, w, w / 2, tint);
    }
}
