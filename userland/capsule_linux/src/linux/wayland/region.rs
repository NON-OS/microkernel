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

//! wl_region: the parts of a surface a client calls opaque, or that take
//! input. A guest's window takes input everywhere, so an input region
//! changes nothing. An opaque region is kept (region_cover.rs): one that
//! covers a pinned window's whole surface lets the compositor put its
//! pixels down unblended (present/pinned/show.rs), as wl_surface allows
//! for the opaque region. A region stays an object either way: a client
//! names it in later requests, destroys it, and reuses its id.

use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::req;
use super::region_cover::{Step, Steps};
use super::scene::Scene;

pub fn create(guest: &mut Guest, args: &mut Args<'_>) {
    let Some(id) = args.u32() else { return };
    if guest.objects.put(id, Object::Region) {
        guest.scene.regions.retain(|r| r.0 != id);
        guest.scene.regions.push((id, Steps::default()));
    }
}

/// wl_region.add or subtract on `id`.
pub fn step(guest: &mut Guest, id: u32, opcode: u16, args: &mut Args<'_>) {
    let (Some(x), Some(y), Some(w), Some(h)) = (args.u32(), args.u32(), args.u32(), args.u32())
    else {
        return;
    };
    let rect = (x as i32, y as i32, w as i32, h as i32);
    if let Some(r) = guest.scene.regions.iter_mut().find(|r| r.0 == id) {
        r.1.push(Step { add: opcode == req::REGION_ADD, rect });
    }
}

/// wl_surface.set_opaque_region on `surface`: the region as it is now, or
/// none for a null one, kept for the surface's next commit.
pub fn set_opaque(guest: &mut Guest, surface: u32, args: &mut Args<'_>) {
    let Some(region) = args.u32() else { return };
    let steps = match region {
        0 => Steps::default(),
        r => match guest.scene.regions.iter().find(|g| g.0 == r) {
            Some(g) => g.1,
            None => {
                let message = alloc::format!("object {r} is not a wl_region");
                return super::display_error::fatal(guest, surface, &message);
            }
        },
    };
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
        s.opaque_said = Some(steps);
    }
}

/// The surface's commit applies the opaque region it was last given.
pub fn commit(guest: &mut Guest, surface: u32) {
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
        if let Some(steps) = s.opaque_said.take() {
            s.opaque = steps;
        }
    }
}

/// Whether `surface`'s opaque region holds the whole of its buffer, `w` by
/// `h` buffer pixels. The region is in surface pixels: a viewport's
/// destination when the surface has one (wp_viewporter: the surface is
/// that size, whatever the buffer's), else a buffer scale's share of the
/// buffer's, rounded up so a part pixel is held too.
pub fn opaque_whole(scene: &Scene, surface: u32, (w, h): (u32, u32)) -> bool {
    let Some(s) = scene.surfaces.iter().find(|s| s.id == surface) else { return false };
    let (sw, sh) = match s.scale.dest {
        Some(dest) => dest,
        None => {
            let by = s.scale.buffer.max(1);
            (w.div_ceil(by), h.div_ceil(by))
        }
    };
    s.opaque.covers(sw, sh)
}
