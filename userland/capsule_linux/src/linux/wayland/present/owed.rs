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

//! A draw the compositor could not take yet: no buffer was released in
//! time (it composes nothing while the lid is shut or the screen blank), or
//! it refused the present. The window keeps what it owes and the service
//! loop draws it again, first after a refresh, then less often, up to the
//! loop's own wait.
//!
//! A guest's commit owed keeps the guest's buffer: its wl_buffer.release
//! waits until the buffer is drawn from, so the client never writes the
//! pixels the window is still to show. A later commit of another buffer
//! takes its place, and the one kept is released then.

use alloc::vec::Vec;

use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::super::ops::ev;
use super::super::out::Event;
use super::super::scene_window::Window;
use super::draw::Drew;

const CLOCK_MONOTONIC: u64 = 1;
/// The first retry after a refresh at 60 Hz, then twice as long each time,
/// up to the loop's own wait (serve/loop_impl.rs).
const FIRST_MS: u64 = 16;
const LAST_MS: u64 = 250;

/// What the window owes, from the guest's buffer or from its own state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Owed {
    /// Drawn again from what the window keeps: its frame and the guest's
    /// pixels beside it, its subsurfaces' kept base.
    Repaint,
    /// The guest's committed `buffer`, drawn whole, then released.
    Guest { buffer: u32 },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Owing {
    pub what: Owed,
    /// Retries so far, and when the next is due.
    pub tries: u32,
    pub due_ms: u64,
}

/// How long after `tries` retries the next is due.
pub fn backoff(tries: u32) -> u64 {
    FIRST_MS.checked_shl(tries.min(8)).map_or(LAST_MS, |ms| ms.min(LAST_MS))
}

/// The window owes `what` too; a guest's buffer is owed over a repaint,
/// since drawing it draws the window whole.
pub fn owe(win: &mut Window, what: Owed) {
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    win.owing = Some(match win.owing {
        None => Owing { what, tries: 0, due_ms: now.saturating_add(backoff(0)) },
        Some(o) => Owing { what: worse(o.what, what), ..o },
    });
}

fn worse(a: Owed, b: Owed) -> Owed {
    match (a, b) {
        (_, Owed::Guest { .. }) => b,
        (Owed::Guest { .. }, Owed::Repaint) => a,
        (Owed::Repaint, Owed::Repaint) => a,
    }
}

/// A repaint from the window's own state was shown: a repaint owed is paid;
/// a guest's buffer owed is still to be drawn.
pub fn paid_repaint(win: &mut Window) {
    if win.owing.is_some_and(|o| o.what == Owed::Repaint) {
        win.owing = None;
    }
}

/// The guest's buffer a debt keeps, if any.
pub fn kept(win: &Window) -> Option<u32> {
    match win.owing?.what {
        Owed::Guest { buffer } => Some(buffer),
        Owed::Repaint => None,
    }
}

/// The client destroyed `buffer`: a debt that kept it can no longer draw
/// it, and the picture stays as it was until its next commit.
pub fn buffer_gone(guest: &mut Guest, buffer: u32) {
    let scene = &mut guest.scene;
    for win in core::iter::once(&mut scene.win).chain(scene.behind.iter_mut()) {
        if kept(win) == Some(buffer) {
            win.owing = None;
        }
    }
}

/// How long until a window's debt is due.
pub fn next_ms(guest: &Guest, now: u64) -> Option<u64> {
    let scene = &guest.scene;
    let wins = core::iter::once(&scene.win).chain(scene.behind.iter());
    wins.filter_map(|w| w.owing).map(|o| o.due_ms.saturating_sub(now)).min()
}

/// Every window whose debt is due drawn again.
pub fn settle(guest: &mut Guest, now: u64) {
    let scene = &guest.scene;
    let wins = core::iter::once(&scene.win).chain(scene.behind.iter());
    let due = |w: &&Window| w.owing.is_some_and(|o| o.due_ms <= now);
    let ids: Vec<u32> = wins.filter(due).filter_map(|w| w.window).collect();
    for id in ids {
        if super::super::window_front::select_window(guest, id) {
            retry(guest, now);
        }
    }
}

fn retry(guest: &mut Guest, now: u64) {
    let (Some(o), Some(shown)) = (guest.scene.win.owing, guest.scene.win.shown) else { return };
    let drew = match o.what {
        Owed::Guest { buffer } => super::show::present(guest, shown.surface, buffer),
        Owed::Repaint => repaint(guest),
    };
    match drew {
        // Held: the debt's buffer is the layer's now, given back by pinned/.
        Drew::Held => guest.scene.win.owing = None,
        Drew::Shown => {
            if let Owed::Guest { buffer } = o.what {
                release(guest, buffer);
            }
        }
        // The draw could not be made from what there is: nothing is left
        // to draw it from, and the next commit draws the window whole.
        Drew::Nothing => {
            guest.scene.win.owing = None;
            if let Owed::Guest { buffer } = o.what {
                release(guest, buffer);
            }
        }
        Drew::Owed => {
            let tries = o.tries.saturating_add(1);
            if let Some(again) = guest.scene.win.owing.as_mut() {
                (again.tries, again.due_ms) = (tries, now.saturating_add(backoff(tries)));
            }
        }
    }
}

/// The window drawn whole from what it keeps.
fn repaint(guest: &mut Guest) -> Drew {
    let scene = &guest.scene;
    let layered = scene.win.shown.is_some_and(|s| scene.subs.shown_on(s.surface));
    if layered || !scene.win.base.is_empty() {
        return super::super::subsurface::compose::repaint(guest);
    }
    super::frame::redraw::redraw(&mut guest.scene)
}

/// `buffer` given back to the client, while it has it.
pub fn release(guest: &mut Guest, buffer: u32) {
    if guest.scene.buffers.iter().any(|b| b.id == buffer) {
        Event::new(buffer, ev::BUFFER_RELEASE).send(&mut guest.display.to_client);
    }
}
