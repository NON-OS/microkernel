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

//! wp_viewporter and wp_fractional_scale_manager_v1: the scale a client
//! draws a surface at (present/scale_math.rs). A viewport's destination is
//! the surface's logical size; its source crops nothing. A fractional scale
//! is told the display's as it is made. False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::output::*;
use super::super::out::Event;
use super::super::present::scale_math::display;
use super::super::present::scaled::quarters;
use super::scale_surface::on;

pub fn route(g: &mut Guest, object: Option<Object>, id: u32, op: u16, a: &mut Args<'_>) -> bool {
    use Object::*;
    match (object, op) {
        (Some(Viewporter), VIEWPORTER_GET_VIEWPORT) => made(g, a, Viewport),
        (Some(FractionalManager), FRACTIONAL_MANAGER_GET) => made(g, a, FractionalScale),
        (Some(Viewport), VIEWPORT_SET_SOURCE) => {}
        (Some(Viewport), VIEWPORT_SET_DESTINATION) => {
            let (w, h) = (a.u32().unwrap_or(0) as i32, a.u32().unwrap_or(0) as i32);
            let dest = (w > 0 && h > 0).then_some((w as u32, h as u32));
            on(g, |s| s.scale.viewport == id, |s| s.dest = dest);
        }
        /* A surface whose viewport goes is shown at its buffer's size again. */
        (Some(Viewport), VIEWPORT_DESTROY) => {
            on(g, |s| s.scale.viewport == id, |s| (s.viewport, s.dest) = (0, None));
            super::super::forget::forget(g, object, id);
        }
        (Some(Viewporter), VIEWPORTER_DESTROY)
        | (Some(FractionalManager), FRACTIONAL_MANAGER_DESTROY)
        | (Some(FractionalScale), FRACTIONAL_DESTROY) => {
            super::super::forget::forget(g, object, id)
        }
        _ => return false,
    }
    true
}

/// A surface's viewport, kept with it, or its fractional scale, told the
/// display's at once: the one scale there is.
fn made(g: &mut Guest, a: &mut Args<'_>, what: Object) {
    let (Some(id), Some(surface)) = (a.u32(), a.u32()) else { return };
    if !g.objects.put(id, what) {
        return;
    }
    match what {
        Object::Viewport => on(g, |s| s.id == surface, |s| s.viewport = id),
        _ => Event::new(id, FRACTIONAL_PREFERRED_SCALE)
            .u32(display(quarters()))
            .send(&mut g.display.to_client),
    }
}
