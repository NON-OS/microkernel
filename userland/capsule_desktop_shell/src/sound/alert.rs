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

//! A tone for the toasts that are not news.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use nonos_policy_client::lookup;

use crate::state::NotifyLevel;

const ALERT_HZ: u32 = 880;
const ALERT_MS: u32 = 90;

const EVERY: u32 = 8;

static DUE: AtomicBool = AtomicBool::new(false);
static PORT: AtomicU32 = AtomicU32::new(0);
static TICKS: AtomicU32 = AtomicU32::new(0);

pub fn mark(level: NotifyLevel) {
    if matches!(level, NotifyLevel::Info) {
        return;
    }
    DUE.store(true, Ordering::Relaxed);
}

/* `policy_ok`: the policy store answered this tick, so its levels may be read
 * without waiting on a store that has gone quiet (server/runner/refresh_clock.rs). */
pub fn service(policy_ok: bool) {
    if policy_ok {
        follow();
    }
    if !DUE.swap(false, Ordering::Relaxed) || !super::levels::alerts_on() {
        return;
    }
    if let Some(gain) = super::levels::gain() {
        super::play::play(ALERT_HZ, ALERT_MS, gain);
    }
}

fn follow() {
    if TICKS.fetch_add(1, Ordering::Relaxed) % EVERY != 0 {
        return;
    }
    let mut port = PORT.load(Ordering::Relaxed);
    if port == 0 {
        port = match lookup() {
            Some(found) => found,
            None => return,
        };
        PORT.store(port, Ordering::Relaxed);
    }
    super::levels::follow(port);
}
