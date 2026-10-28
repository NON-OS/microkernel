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

//! The frame the shown window has now, decided in one place for the
//! pixels and the pointer alike.

use nonos_app_skeleton::runner::chrome::quarters_for;

use super::super::super::fit::Mode;
use super::super::super::reshape::Shape;
use super::super::super::scene::Scene;
use super::geometry::{around_at, Around};

/// The frame around content of `shape`, for the buffer drawn now: none
/// when it is full screen, or is drawn for a configure that is.
pub fn around_shape(scene: &Scene, (w, h, _): Shape) -> Around {
    let mode = scene.fit.due().map_or(scene.fit.now(), |c| c.mode);
    let (bare, max) = (mode.bare() || scene.frame.own_frame, mode == Mode::Maximized);
    around_at(w, h, scene.frame.quarters, bare, max)
}

/// The content rect a frame leaves of `rect`, the window's whole on screen.
pub fn inset(scene: &Scene, (x, y, w, h): (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    let a = around_at(0, 0, scene.frame.quarters, scene.frame.own_frame, true);
    (x, y, w.saturating_sub(a.w).max(1), h.saturating_sub(a.h).max(1))
}

/// The shown window's frame and its content size, or None with no window.
pub fn shown(scene: &Scene) -> Option<(Around, Shape)> {
    let shape = scene.shape?;
    scene.out?;
    Some((around_shape(scene, shape), shape))
}

/// The shown window's rect on screen, frame and all, as the compositor and
/// the window manager are told it.
pub fn outer_rect(scene: &Scene) -> Option<(u32, u32, u32, u32)> {
    let ((x, y), (a, _)) = (scene.at?, shown(scene)?);
    Some((x, y, a.w, a.h))
}

/// (`x`, `y`) on the window as a point on the guest's content, or None on
/// the frame.
pub fn to_content(scene: &Scene, x: i32, y: i32) -> Option<(i32, i32)> {
    let Some((a, (cw, ch, _))) = shown(scene) else { return Some((x, y)) };
    a.on_content(x, y, cw, ch)
}

/// The display's scale, read once as the window opens.
pub fn note_scale(scene: &mut Scene) {
    if scene.window.is_none() {
        let (w, h) = super::super::super::output::screen();
        scene.frame.quarters = quarters_for(w, h);
    }
}
