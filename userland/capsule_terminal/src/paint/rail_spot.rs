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

//! Which control of the rail's lists lies under a point, and the rows a
//! point is tested against: the same boxes the lists are drawn in.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use super::rail_nav::{row_h, Nav};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spot {
    NewSession,
    AddProject,
    Session(u32),
    Project(u32),
}

/// Row `i` of a list.
pub fn row(list: Rect, i: u32) -> Rect {
    Rect { y: list.y + i * row_h(), h: row_h(), ..list }
}

/// The control under the content point `(x, y)`.
pub fn spot(n: &Nav, sessions: u32, projects: u32, x: i32, y: i32) -> Option<Spot> {
    let at = |list: Rect, count: u32| (0..count).find(|&i| contains(row(list, i), x, y));
    if contains(n.s_plus, x, y) {
        return Some(Spot::NewSession);
    }
    if contains(n.p_plus, x, y) {
        return Some(Spot::AddProject);
    }
    at(n.s_list, sessions).map(Spot::Session).or_else(|| at(n.p_list, projects).map(Spot::Project))
}

/// Last path component, or the whole path when it names the root.
pub fn base_name(path: &str) -> &str {
    match path.rsplit('/').next() {
        Some(tail) if !tail.is_empty() => tail,
        _ => path,
    }
}
