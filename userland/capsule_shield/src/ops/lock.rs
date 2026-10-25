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

//! Locking the shield: the open store stops being open at once.

use std::sync::atomic::Ordering;
use std::sync::Arc;

use shield_wire::Values;

use super::held::{CANCEL, LOCKS, OPEN};
use super::seen::SEEN;
use super::Answer;

/// Answered at once, on the service loop: the store stops being the open
/// one now, a proof under way is stopped, and its keys are dropped off the
/// loop, since a proof or a scan may hold the store for minutes yet.
pub fn lock() -> Answer {
    LOCKS.fetch_add(1, Ordering::SeqCst);
    let open = OPEN.lock().map_err(|_| String::from("the shield is busy"))?.take();
    if let Ok(mut seen) = SEEN.lock() {
        *seen = None;
    }
    if let Ok(c) = CANCEL.lock() {
        if let Some(t) = c.as_ref() {
            t.cancel();
        }
    }
    if let Some(open) = open {
        let wallet = Arc::clone(&open.wallet);
        let spawned = std::thread::Builder::new().name(String::from("lock")).spawn(move || {
            let _ = wallet.lock();
        });
        if spawned.is_err() {
            // No thread: the keys are dropped here, which may wait on the store.
            let _ = open.wallet.lock();
        }
    }
    Ok(Values::new())
}
