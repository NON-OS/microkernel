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
use super::present::draw::Drew;
use super::present::owed;
use super::subsurface::commit::sub_commit;
use super::subsurface::compose::repaint;
use super::subsurface::restack::parent_commit;
use super::window_front::Front;

#[path = "damage_show.rs"]
pub mod damage_show;
#[path = "launch_clock.rs"]
pub mod launch_clock;
#[path = "pace.rs"]
pub mod pace;

/// A commit with a buffer is the only thing that reaches the screen, and
/// only a window's, the surface of a toplevel, with its subsurfaces laid
/// on it (subsurface/). A cursor's buffer is released and its frames
/// answered, so its client draws on, and never takes the window's place.
/// A buffer is released once, at the commit that applies its attach; a
/// null buffer attached to a window's surface unmaps it (toplevel/unmap.rs),
/// and one attached to a shown popup's ends the popup (popup/gone.rs).
/// A toplevel's commit goes to its own window, a window of its own on screen
/// beside the guest's others (window_front/).
pub fn commit(guest: &mut Guest, id: u32) {
    // The window this surface is shown on is the one acted on.
    super::window_front::select_for(guest, id);
    let popup = super::popup::gone::shown_on(&guest.scene, id);
    super::toplevel::first(guest, id);
    super::popup::show::first(guest, id);
    super::region::commit(guest, id);
    // A buffer attached since the last commit, null or not, which this
    // commit applies: only then is there one to release, or to unmap with.
    let fresh = guest.scene.surfaces.iter().find(|s| s.id == id).is_some_and(|s| s.attached);
    let changed = sub_commit(guest, id) == Some(true);
    parent_commit(&mut guest.scene.subs, id);
    let Some((buffer, window)) = surface_buffer(guest, id) else {
        if changed {
            repaint(guest);
        }
        if fresh {
            super::toplevel::unmap(guest, id);
            if let Some(popup) = popup {
                super::popup::gone::unmapped(guest, popup);
            }
        }
        applied(guest, id);
        super::presentation::committed(guest, id);
        return;
    };
    // The buffers this commit gives back to the client once it is done.
    let mut done: [Option<u32>; 2] = [fresh.then_some(buffer), None];
    if window {
        // A new toplevel's window opens beside the others (window_front/).
        match super::window_front::front(guest, id) {
            Front::Full => full(),
            Front::Shown | Front::New => {
                done = shown(guest, id, buffer, fresh);
                super::output_enter::shown(guest);
                super::toplevel::held(guest, id);
            }
        }
    } else if super::popup::present::present(guest, id, buffer) {
        // A popup in a window of its own (popup/window.rs).
    } else if changed {
        repaint(guest);
    }
    for b in done.into_iter().flatten() {
        Event::new(b, ev::BUFFER_RELEASE).send(&mut guest.display.to_client);
    }
    applied(guest, id);
    super::presentation::committed(guest, id);
    let owed = take_frames(guest, id);
    pace::owe(guest, id, owed);
}

/// `buffer` committed on the window `id` shown, and the buffers given back
/// for it: this one when it was attached afresh, and one a debt kept from
/// before (present/owed.rs). A draw that found no buffer free keeps this
/// one instead, released once it is drawn from, so the client does not
/// write the pixels the window is still to show.
fn shown(guest: &mut Guest, id: u32, buffer: u32, fresh: bool) -> [Option<u32>; 2] {
    let kept = owed::kept(&guest.scene.win);
    let drew = damage_show::present(guest, id, buffer);
    let keep = matches!(drew, Drew::Owed | Drew::Held).then_some(buffer);
    if let Some(b) = keep.filter(|_| drew == Drew::Owed) {
        owed::owe(&mut guest.scene.win, owed::Owed::Guest { buffer: b });
    }
    let this = (fresh || kept == Some(buffer)).then_some(buffer).filter(|b| keep != Some(*b));
    let before = kept.filter(|b| *b != buffer);
    // A buffer given back is drawn from no more: this commit's took its place.
    if before.is_some() && owed::kept(&guest.scene.win) == before {
        guest.scene.win.owing = None;
    }
    [this, before]
}

/// The attach is applied: a commit after has none of its own.
fn applied(guest: &mut Guest, id: u32) {
    if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) {
        s.attached = false;
    }
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

/// A toplevel past MOST_WINDOWS: its commits are answered, never shown, and
/// said once, not at every frame.
fn full() {
    use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
    static SAID: AtomicBool = AtomicBool::new(false);
    if !SAID.swap(true, Relaxed) {
        crate::linux::say::say(b"[WAYLAND] a toplevel past the windows a client may show: not shown\n");
    }
}
