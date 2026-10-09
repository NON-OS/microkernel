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

//! Everything a shadowed panel paints, for the damage that shows it.
//!
//! `shadow_panel` draws the panel's soft shadow as rings up to `spread`
//! pixels outside its rectangle. The dock's damage named the dock's
//! rectangle only, so when the dock hid the compositor recomposed the panel
//! away and left the shadow's rings on screen: a faint outline of a dock
//! that was no longer there, until some other damage happened to cover it.

/// The rectangle (`x`, `y`, `width`, `height`) grown by `spread` on every
/// side, clipped to a `display_w` by `display_h` display.
pub fn with_shadow(
    r: (u32, u32, u32, u32),
    spread: u32,
    display_w: u32,
    display_h: u32,
) -> (u32, u32, u32, u32) {
    let (x0, y0) = (r.0.saturating_sub(spread), r.1.saturating_sub(spread));
    let x1 = r.0.saturating_add(r.2).saturating_add(spread).min(display_w);
    let y1 = r.1.saturating_add(r.3).saturating_add(spread).min(display_h);
    (x0, y0, x1.saturating_sub(x0), y1.saturating_sub(y0))
}
