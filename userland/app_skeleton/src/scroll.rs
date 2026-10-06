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

//! The wheel's one rule for a view that scrolls. A wheel event carries its
//! notches in `delta_y`, signed as the input ring signs them for every device
//! (USB and I2C HID mice, the PS/2 wheel, a touchpad's two-finger scroll):
//! positive is a notch away from the user, and moves the view toward the top.
//! A view keeps its own offset and its own end; this moves the offset a step
//! per notch and holds it inside `0..=max`, so no view scrolls past its
//! content and no two views disagree on direction. It names nothing else in
//! the crate, so the proofs crates mount it by path beside the app source
//! that calls it.

/// Lines or rows a notch moves a text or list view.
pub const WHEEL_LINES: usize = 3;

/// The most notches one event is taken for. A touchpad flick can post a
/// large step at once; past this it would jump whole pages.
pub const MAX_NOTCHES: u32 = 10;

/// Where a view at `offset`, which may scroll as far as `max`, goes for a
/// wheel `delta_y`, moving `step` per notch.
pub fn wheel_offset(offset: usize, delta_y: i32, step: usize, max: usize) -> usize {
    let notches = delta_y.unsigned_abs().min(MAX_NOTCHES) as usize;
    let travel = notches.saturating_mul(step);
    let next =
        if delta_y > 0 { offset.saturating_sub(travel) } else { offset.saturating_add(travel) };
    next.min(max)
}

/// `wheel_offset` for a view that scrolls in pixels.
pub fn wheel_px(offset: u32, delta_y: i32, step: u32, max: u32) -> u32 {
    wheel_offset(offset as usize, delta_y, step as usize, max as usize) as u32
}
