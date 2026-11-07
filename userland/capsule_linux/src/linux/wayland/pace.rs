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

//! Frame callbacks at most once a display refresh (pace_rule.rs).
//! A client answered the moment it commits draws its next frame at once, and
//! one that draws every turn of its loop (DarkFi's GUI on Mesa's software
//! driver, the box GL design) spends a CPU on frames no one sees. Held until
//! the next refresh after the surface's last answer, it draws no faster than
//! the display shows, and a video player is answered for every frame of a
//! 60 a second video; an idle window is still answered at once.

use alloc::vec::Vec;

use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::super::ops::ev;
use super::super::out::Event;

#[path = "pace_rule.rs"]
mod pace_rule;
use pace_rule::{due, next};
pub use pace_rule::{period, refresh_mhz};

const CLOCK_MONOTONIC: u64 = 1;

/// `callbacks`, owed after `id`'s present: answered now, or held for its
/// period. Without a clock they are answered now, never held for good.
pub fn owe(guest: &mut Guest, id: u32, callbacks: Vec<u32>) {
    let p = period(guest.scene.refresh_ns);
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) else { return };
    s.held.extend(callbacks);
    let now = now_ms(CLOCK_MONOTONIC);
    if now.is_none_or(|now| due(s.last_done, now, p) <= now) {
        answer(guest, id, now.unwrap_or(0));
    }
}

/// Every surface whose period has passed answered, and every window's
/// draw owed that is due drawn (present/owed.rs).
pub fn settle(guest: &mut Guest, now: u64) {
    let p = period(guest.scene.refresh_ns);
    let ready =
        |s: &&super::super::state::Surface| !s.held.is_empty() && due(s.last_done, now, p) <= now;
    let ids: Vec<u32> = guest.scene.surfaces.iter().filter(ready).map(|s| s.id).collect();
    ids.into_iter().for_each(|id| answer(guest, id, now));
    super::super::presentation_wait::settle(guest, now);
    super::super::present::owed::settle(guest, now);
    super::super::present::pinned::held::settle(guest, now);
}

/// How long until a held callback, or a window's draw owed, is due.
pub fn next_ms(guest: &Guest, now: u64) -> Option<u64> {
    let held = guest.scene.surfaces.iter().filter(|s| !s.held.is_empty());
    let frames = next(held.map(|s| s.last_done), now, period(guest.scene.refresh_ns));
    let shown = super::super::presentation_wait::next_ms(guest, now);
    let owed = super::super::present::owed::next_ms(guest, now);
    let pinned = super::super::present::pinned::held::next_ms(guest, now);
    frames.into_iter().chain(shown).chain(owed).chain(pinned).min()
}

fn answer(guest: &mut Guest, id: u32, now: u64) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) else { return };
    let held = core::mem::take(&mut s.held);
    s.last_done = Some(now);
    for callback in held {
        Event::new(callback, ev::CALLBACK_DONE).u32(now as u32).send(&mut guest.display.to_client);
        /* A callback ends with its done, and its id goes back to the client. */
        guest.objects.drop_id(callback);
        super::super::handlers::delete_id(guest, callback);
    }
}
