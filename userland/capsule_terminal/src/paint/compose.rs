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

use nonos_app_skeleton::PaintBuffer;

use super::rail_left;
use crate::layout::Layout;
use crate::palette::{Index, Palette};
use crate::rail::Rail;
use crate::term::prefs::types::Project;
use crate::term::state::State;
use crate::term::theme::types::Theme;

#[path = "compose_tab.rs"]
mod compose_tab;
use compose_tab::paint;

/// What a frame paints around the active tab: every tab, which one is active,
/// the rail and its pinned paths, and whether the palette is up.
pub struct Scene<'a> {
    pub tabs: &'a [State],
    pub active: usize,
    pub rail: &'a Rail,
    pub projects: &'a [Project],
    pub monitor: bool,
    pub scroll: u32,
    pub pal: &'a Palette,
    pub rail_open: bool,
}

pub fn paint_tabs(scene: &Scene, fb: &mut PaintBuffer, t: &Theme, font_scale: u32) -> Layout {
    let Scene { tabs, active, projects, pal, rail_open, .. } = *scene;
    let l = paint(&tabs[active], fb, t, font_scale, rail_open);
    if l.left_rail.w > 0 {
        rail_left::draw(fb, l.left_rail, scene, t);
    }
    if pal.open {
        let ix = Index::build(tabs, active, projects);
        super::palette::draw(fb, l.body, pal, &ix, t);
    }
    l
}
