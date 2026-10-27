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

//! What a wl_surface hears and says of its scale: the whole scale, told as
//! it is made (wl_compositor 6), and set_buffer_scale's factor.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::ev;
use super::super::out::Event;
use super::super::present::scale_math::{whole, Scale};
use super::super::present::scaled::quarters;
use super::super::state::Surface;

/// Change the scale of the surfaces `which` picks.
pub fn on(g: &mut Guest, which: impl Fn(&Surface) -> bool, change: impl Fn(&mut Scale)) {
    for s in g.scene.surfaces.iter_mut().filter(|s| which(s)) {
        change(&mut s.scale);
    }
}

/// set_buffer_scale. A factor below one is a protocol error, and here
/// changes nothing.
pub fn buffer_scale(g: &mut Guest, surface: u32, a: &mut Args<'_>) {
    let factor = a.u32().unwrap_or(0) as i32;
    if factor > 0 {
        on(g, |s| s.id == surface, |s| s.buffer = factor as u32);
    }
}

/// preferred_buffer_scale to a surface made at wl_compositor 6 or later.
pub fn told(g: &mut Guest, surface: u32) {
    if g.objects.at_least(surface, 6) {
        Event::new(surface, ev::SURFACE_PREFERRED_BUFFER_SCALE)
            .u32(whole(quarters()))
            .send(&mut g.display.to_client);
    }
}
