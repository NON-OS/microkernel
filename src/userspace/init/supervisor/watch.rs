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

//! Watching the core services and restarting one that ended
//! (`watch_rule.rs`), once a second from init's loop.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::watch_rule::{is_watched, next, Next};
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::services::lifecycle::CapsuleState;

#[derive(Clone, Copy)]
struct Watched {
    prefix: &'static str,
    spawn_fn: fn() -> Result<(), SpawnError>,
    state_fn: fn() -> &'static CapsuleState,
    gave_up: bool,
    /// When it was first seen ended, 0 while it runs.
    dead_since: u64,
}

static WATCHED: Mutex<Vec<Watched>> = Mutex::new(Vec::new());

/// Watch `name` if it is one of the core services, once it has started.
pub(crate) fn watch(
    prefix: &'static str,
    name: &str,
    spawn_fn: fn() -> Result<(), SpawnError>,
    state_fn: fn() -> &'static CapsuleState,
) {
    if !is_watched(name) {
        return;
    }
    let mut list = WATCHED.lock();
    if list.iter().any(|w| w.state_fn as usize == state_fn as usize) {
        return;
    }
    list.push(Watched { prefix, spawn_fn, state_fn, gave_up: false, dead_since: 0 });
}

pub(crate) fn tick(now_ms: u64) {
    let list: Vec<Watched> = WATCHED.lock().clone();
    for (i, w) in list.iter().enumerate() {
        let state = (w.state_fn)();
        let alive = state.is_alive();
        let dead_since = match (alive, w.dead_since) {
            (true, _) => 0,
            (false, 0) => now_ms.max(1),
            (false, since) => since,
        };
        if dead_since != w.dead_since {
            if let Some(entry) = WATCHED.lock().get_mut(i) {
                entry.dead_since = dead_since;
            }
        }
        let started = state.generation() != 0;
        let spent = state.restart_count() >= state.max_restarts();
        let dead_for = if alive { 0 } else { now_ms.saturating_sub(dead_since) };
        match next(alive, started, state.should_respawn(now_ms), spent, dead_for) {
            Next::Leave => {}
            Next::Restart => {
                state.record_exit(now_ms);
                if let Some(entry) = WATCHED.lock().get_mut(i) {
                    entry.dead_since = 0;
                }
                super::super::capsule_boot::restart(w.prefix, w.spawn_fn);
            }
            Next::GiveUp if !w.gave_up => {
                crate::sys::boot_log::warn(&alloc::format!(
                    "{}: ended again after {} restarts; not restarted until reboot",
                    w.prefix,
                    state.restart_count()
                ));
                if let Some(entry) = WATCHED.lock().get_mut(i) {
                    entry.gave_up = true;
                }
            }
            Next::GiveUp => {}
        }
    }
}
