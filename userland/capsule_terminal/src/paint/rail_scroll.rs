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
use super::rail_column::Column;
use super::rail_geom::telemetry_h;
use super::rail_nav::nav_h;
use super::rail_text::gap;
use crate::layout::rows::scroll_clamp;
use crate::layout::Rect;

/// Everything the column's height depends on. The painter and the hit-test both
/// build one of these from the same state, so both resolve the same offset.
#[derive(Clone, Copy)]
pub struct RailFit {
    pub sessions: u32,
    pub projects: u32,
    pub telemetry: bool,
}

pub fn content_h(f: RailFit) -> u32 {
    let nav = nav_h(f.sessions, f.projects);
    if !f.telemetry {
        return nav;
    }
    nav + gap() + telemetry_h()
}

pub fn clamp(offset: u32, f: RailFit, viewport: u32) -> u32 {
    scroll_clamp(offset, content_h(f), viewport)
}

/// Content top of the telemetry block, a gap under the lists.
pub fn telemetry_top(f: RailFit) -> u32 {
    nav_h(f.sessions, f.projects) + gap()
}

/// The rail in `rect` scrolled by `scroll`, clamped once here so the painter,
/// the hover and the click all place the column at the same offset.
pub fn column(rect: Rect, scroll: u32, f: RailFit) -> Column {
    Column { rect, off: clamp(scroll, f, rect.h), sessions: f.sessions, projects: f.projects }
}
