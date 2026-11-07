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

//! A popup's place is its window geometry's (xdg_positioner places that
//! box), and its surface starts at that box's corner up and left of it
//! (present/crop.rs). A GTK menu drawn with a shadow round it sets its
//! geometry inside the buffer; reckoned from the buffer's corner it showed
//! a shadow's width right of and below its anchor, and the pointer was told
//! the same distance off on it.

use super::super::present::crop::corner;
use super::super::scene::Scene;

/// The corner of `surface`'s window geometry on the surface.
fn of(scene: &Scene, surface: u32) -> (i32, i32) {
    corner(scene.surfaces.iter().find(|s| s.id == surface).and_then(|s| s.geometry))
}

/// A point `at` reckoned from popup `surface`'s place, on the surface. A
/// popup in a window of its own with a viewport is shown whole from its
/// surface's corner (present/scaled.rs crops no viewport), so its points
/// are its window's own.
pub fn on_surface(scene: &Scene, surface: u32, (x, y): (i32, i32)) -> (i32, i32) {
    let own_window = scene.popup_windows.iter().any(|w| w.surface == surface);
    let s = scene.surfaces.iter().find(|s| s.id == surface);
    if own_window && s.is_some_and(|s| s.scale.dest.is_some()) {
        return (x, y);
    }
    let (cx, cy) = of(scene, surface);
    (x.saturating_add(cx), y.saturating_add(cy))
}

/// Where popup `surface`'s surface starts when its place is at `at`.
pub fn surface_at(scene: &Scene, surface: u32, (x, y): (i32, i32)) -> (i32, i32) {
    let (cx, cy) = of(scene, surface);
    (x.saturating_sub(cx), y.saturating_sub(cy))
}
