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

//! A commit on a subsurface, and what a commit on a parent applies. A
//! synchronized subsurface's commit waits for its parent's next commit;
//! a desynchronized one's shows at once. Either way its pixels are copied
//! out of the guest now, since its buffer is released now.

use alloc::string::String;
use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::super::frame_len::MAX_FRAME;
use super::super::present::source::{source, take};
use super::super::present::surface::say;
use super::blend::Plane;
use super::state::Kept;

/// The most bytes of subsurface pixels kept at once, waiting or shown: one
/// frame's, so many subsurfaces ask this capsule for no more than a window.
const KEPT_MAX: usize = MAX_FRAME as usize;

/// None when `surface` is no subsurface; else whether what it shows changed.
pub fn sub_commit(guest: &mut Guest, surface: u32) -> Option<bool> {
    let buffer = guest.scene.surfaces.iter().find(|s| s.id == surface)?.pending;
    let sub = guest.scene.subs.of_mut(surface)?;
    let sync = sub.sync;
    // What this commit replaces goes first, so it is not counted below.
    match sync {
        true => sub.waiting = None,
        false => sub.shown = None,
    }
    let kept = buffer.and_then(|b| capture(guest, b));
    let sub = guest.scene.subs.of_mut(surface)?;
    match sync {
        true => sub.waiting = Some(kept),
        false => sub.shown = kept,
    }
    Some(!sync)
}

/// The pixels of `buffer`, if they fit; else said, and none.
fn capture(guest: &Guest, buffer: u32) -> Option<Kept> {
    let src = source(&guest.scene, buffer)
        .map_err(|why| say(String::from_utf8_lossy(why).into()))
        .ok()?;
    if guest.scene.subs.kept().saturating_add(src.bytes) > KEPT_MAX {
        say("[WAYLAND] subsurface pixels over the budget, not shown\n".into());
        return None;
    }
    let mut px = Vec::new();
    px.try_reserve_exact(src.bytes).ok()?;
    px.resize(src.bytes, 0);
    let (w, h, stride) = src.shape;
    take(guest.pid, &src, &mut px).then_some(Kept { px, plane: Plane { w, h, stride } })
}
