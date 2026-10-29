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

//! `wl_surface.commit`: the only request that reaches the screen.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::ops::ev;
use super::out::Event;
use super::subsurface::commit::sub_commit;
use super::subsurface::compose::repaint;
use super::subsurface::restack::parent_commit;

#[path = "pace.rs"]
pub mod pace;
#[path = "damage_show.rs"]
pub mod damage_show;
#[path = "launch_clock.rs"]
pub mod launch_clock;

/// A commit with a buffer is the only thing that reaches the screen, and
/// only a window's, the surface of a toplevel, with its subsurfaces laid
/// on it (subsurface/). A cursor's buffer is released and its frames
/// answered, so its client draws on, and never takes the window's place.
pub fn commit(guest: &mut Guest, id: u32) {
    super::popup::gone::unheld_commit(guest, id);
    let changed = sub_commit(guest, id) == Some(true);
    parent_commit(&mut guest.scene.subs, id);
    let Some((buffer, window)) = surface_buffer(guest, id) else {
        if changed {
            repaint(guest);
        }
        return;
    };
    if window {
        damage_show::present(guest, id, buffer);
        super::output_enter::shown(guest);
    } else if changed {
        repaint(guest);
    }
    Event::new(buffer, ev::BUFFER_RELEASE).send(&mut guest.display.to_client);
    let owed = take_frames(guest, id);
    pace::owe(guest, id, owed);
}

fn surface_buffer(guest: &Guest, id: u32) -> Option<(u32, bool)> {
    let s = guest.scene.surfaces.iter().find(|s| s.id == id)?;
    Some((s.pending?, s.toplevel.is_some()))
}

fn take_frames(guest: &mut Guest, id: u32) -> Vec<u32> {
    match guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        Some(s) => core::mem::take(&mut s.frames),
        None => Vec::new(),
    }
}
