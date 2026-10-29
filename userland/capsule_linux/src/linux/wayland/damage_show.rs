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

//! A window's commit shows only what its client said changed. Every commit
//! read the whole buffer out of the guest, blended every pixel into the
//! frame and had the compositor compose the whole window again, so a caret
//! blink cost what the first frame did.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::req;
use super::super::present::show;
use super::launch_clock;

#[path = "damage.rs"]
pub mod damage;
#[path = "damage_part.rs"]
mod damage_part;

use damage::{at_commit, Damage};

/// `wl_surface.damage` or `damage_buffer` on `id`, kept for its commit.
pub fn said(guest: &mut Guest, id: u32, opcode: u16, args: &mut Args<'_>) {
    let (Some(x), Some(y), Some(w), Some(h)) = (args.u32(), args.u32(), args.u32(), args.u32())
    else {
        return;
    };
    let (x, y, w, h) = (x as i32, y as i32, w as i32, h as i32);
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) else { return };
    s.damage = match opcode {
        req::SURFACE_DAMAGE_BUFFER => s.damage.add((x, y, w, h), 1),
        /* In surface pixels: a viewport's are no whole number of the buffer's. */
        _ if s.scale.dest.is_some() => Damage::Whole,
        _ => s.damage.add((x, y, w, h), s.scale.buffer),
    };
}

/// `buffer`, committed on the window's `surface`, shown: only the part
/// said changed when the window is as it was, else whole.
pub fn present(guest: &mut Guest, surface: u32, buffer: u32) {
    let t0 = launch_clock::now();
    let said = match guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
        Some(s) => at_commit(core::mem::take(&mut s.damage), core::mem::take(&mut s.attached)),
        None => Damage::Whole,
    };
    let (read, whole) = match damage_part::part(guest, surface, buffer, said) {
        Some(read) => (read, false),
        None => {
            show::present(guest, surface, buffer);
            (whole_bytes(guest, buffer), true)
        }
    };
    launch_clock::shown(t0, read, whole);
}

fn whole_bytes(guest: &Guest, buffer: u32) -> u64 {
    let b = guest.scene.buffers.iter().find(|b| b.id == buffer);
    b.map_or(0, |b| u64::from(b.stride) * u64::from(b.height))
}
