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

use crate::browser::paint::pages::cells::px;
use crate::browser::paint::pages::group_head;
use crate::browser::paint::pages::pills::{specs, Pill};
use crate::browser::shell::{Page, Shell};
use crate::browser::ui::list_geom::{middle_pills, ListGeom};

const DELETE_GAP: u32 = 16;

/// The pills the header draws in `g`, and their widths. The Tabs page keeps
/// All and New group and as many groups as fit between them, left of
/// Delete group when it is filtered to one.
pub fn row(shell: &Shell, g: &ListGeom) -> (Vec<Pill>, Vec<i32>) {
    let mut specs = specs(shell);
    if shell.page == Page::Tabs {
        let held = group_head::filtered(shell).map_or(0, |_| group_head::delete_rect(g).w);
        let room = g.frame.w - held - if held > 0 { px(DELETE_GAP) } else { 0 };
        let kept = middle_pills(&widths(&specs), room);
        specs.drain(1 + kept..specs.len() - 1);
    }
    let w = widths(&specs);
    (specs, w)
}

/* How wide each pill is drawn, and so where each one is placed. */
fn widths(pills: &[Pill]) -> Vec<i32> {
    pills.iter().map(|p| p.widget().size().0 as i32).collect()
}
