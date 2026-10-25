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

//! Which part of the rail's navigation lists a click landed on.

use super::nav_sections;
use crate::layout::Rect;
use crate::paint::rail_band::hits;
use crate::paint::rail_row::row_at;

pub enum LeftHit {
    NewSession,
    AddProject,
    Session(u32),
    Project(u32),
}

pub fn hit(r: Rect, sessions: u32, projects: u32, off: u32, x: u32, y: u32) -> Option<LeftHit> {
    let s = nav_sections(r, off, sessions, projects);
    let (x, y) = (x.saturating_sub(r.x), y as i32 - r.y as i32);
    if hits(&s.s_plus, x, y) {
        return Some(LeftHit::NewSession);
    }
    if hits(&s.p_plus, x, y) {
        return Some(LeftHit::AddProject);
    }
    if let Some(i) = row_at(&s.s_list, sessions, x, y) {
        return Some(LeftHit::Session(i));
    }
    row_at(&s.p_list, projects, x, y).map(LeftHit::Project)
}
