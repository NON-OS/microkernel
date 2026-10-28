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

//! Frame callbacks at most 30 a second. A client answered the moment it
//! commits draws its next frame at once, and one that draws every turn of
//! its loop (DarkFi's GUI on Mesa's software driver, docs/design/box-gl.md)
//! spends a CPU on frames no one sees. Held until a period has passed since
//! the surface's last answer, it draws 30 a second; an idle window is still
//! answered at once (pace_rule.rs).

use alloc::vec::Vec;

use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::super::ops::ev;
use super::super::out::Event;

#[path = "pace_rule.rs"]
mod pace_rule;
use pace_rule::{due, next};

const CLOCK_MONOTONIC: u64 = 1;

/// `callbacks`, owed after `id`'s present: answered now, or held for its
/// period. Without a clock they are answered now, never held for good.
pub fn owe(guest: &mut Guest, id: u32, callbacks: Vec<u32>) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == id) else { return };
    s.held.extend(callbacks);
    let now = now_ms(CLOCK_MONOTONIC);
    if now.map_or(true, |now| due(s.last_done, now) <= now) {
        answer(guest, id, now.unwrap_or(0));
    }
}

/// Every surface whose period has passed answered.
pub fn settle(guest: &mut Guest, now: u64) {
    let ready =
        |s: &&super::super::state::Surface| !s.held.is_empty() && due(s.last_done, now) <= now;
    let ids: Vec<u32> = guest.scene.surfaces.iter().filter(ready).map(|s| s.id).collect();
    ids.into_iter().for_each(|id| answer(guest, id, now));
}

/// How long until a held callback is due.
pub fn next_ms(guest: &Guest, now: u64) -> Option<u64> {
    next(guest.scene.surfaces.iter().filter(|s| !s.held.is_empty()).map(|s| s.last_done), now)
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
