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

//! The buffer each window shows from its pin, and those taken off a layer
//! the client gets back once the compositor has let them go: polled on the
//! serve loop's turns, as a window's debts are (owed.rs).

use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::super::super::scene_window::Window;
use super::super::owed::release;
use super::pins::spare;
use super::rule::{given_back, gives_back, GIVE_BACK_MS, IDLE_MS};

const CLOCK_MONOTONIC: u64 = 1;

/// How often a buffer waiting to be given back is asked after: a refresh.
const POLL_MS: u64 = 8;

/// A client's buffer a layer draws, or drew.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Held {
    pub buffer: u32,
    pub handle: u64,
    /// Its release count when it went on the layer.
    pub seen: u32,
    /// When it went on the layer.
    pub since: u64,
}

/// `handle`'s release count now, or the kernel's refusal.
pub fn count(handle: u64) -> i64 {
    nonos_libc::mk_surface_fence_wait(nonos_libc::peer::pin_sid(handle), 0, 0)
}

pub fn now() -> u64 {
    now_ms(CLOCK_MONOTONIC).unwrap_or(0)
}

/// How long until a held buffer is to be asked after or given back: IDLE_MS
/// after it was shown, or GIVE_BACK_MS while its client has a spare.
pub fn next_ms(guest: &Guest, now: u64) -> Option<u64> {
    let scene = &guest.scene;
    let poll = (!scene.releasing.is_empty()).then_some(POLL_MS);
    let windows = core::iter::once(&scene.win).chain(scene.behind.iter());
    let wait =
        |w: &Window, h: &Held| if spare(scene, w, h.buffer) { GIVE_BACK_MS } else { IDLE_MS };
    let shown = windows.filter_map(|w| w.content.map(|h| h.since.saturating_add(wait(w, &h))));
    let idle = shown.map(|due| due.saturating_sub(now)).min();
    poll.into_iter().chain(idle).min()
}

/// Every buffer the compositor let go of given back, and every window that
/// showed one buffer for IDLE_MS, its client with no other to draw in,
/// drawn into its own buffers, so its client may draw in that one again.
pub fn settle(guest: &mut Guest, now: u64) {
    let waiting = core::mem::take(&mut guest.scene.releasing);
    for h in waiting {
        if given_back(h.seen, count(h.handle), now.saturating_sub(h.since)) {
            release(guest, h.buffer);
        } else {
            guest.scene.releasing.push(h);
        }
    }
    let scene = &guest.scene;
    let windows = core::iter::once(&scene.win).chain(scene.behind.iter());
    let due = |w: &&super::super::super::scene_window::Window| {
        let h = w.content.filter(|h| gives_back(h.since, now, spare(scene, w, h.buffer)))?;
        Some((w.window?, h.since))
    };
    let ids: alloc::vec::Vec<(u32, u64)> = windows.filter_map(|w| due(&w)).collect();
    for (id, since) in ids {
        if super::super::super::window_front::select_window(guest, id) {
            let _ = super::show::give_back(guest);
            let win = &mut guest.scene.win;
            if win.content.is_none() {
                win.idle_back = Some(since);
            }
        }
    }
}
