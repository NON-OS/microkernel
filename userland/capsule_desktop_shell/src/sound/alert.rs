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

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::state::NotifyLevel;

/// 880 Hz for 90 ms: short enough not to sit over the toast it announces.
const ALERT_HZ: u32 = 880;
const ALERT_MS: u32 = 90;

/// The gain the audio service's own tone uses.
pub(super) const GAIN: u16 = 0x2000;

/// The caller runs about once a second, and nobody moves this switch often.
const EVERY: u32 = 8;

static ENABLED: AtomicBool = AtomicBool::new(false);
static DUE: AtomicBool = AtomicBool::new(false);
static PORT: AtomicU32 = AtomicU32::new(0);
static TICKS: AtomicU32 = AtomicU32::new(0);

// Info is something that happened on its own. Warn and Error are answers to
// what the reader just did, and those are the ones worth a sound.
pub fn mark(level: NotifyLevel) {
    if matches!(level, NotifyLevel::Info) {
        return;
    }
    DUE.store(true, Ordering::Relaxed);
}

/// Clears the flag either way, so a tone marked while the switch was off does
/// not sound the moment it is turned on.
pub fn service() {
    follow();
    if DUE.swap(false, Ordering::Relaxed) && ENABLED.load(Ordering::Relaxed) {
        super::play::play(ALERT_HZ, ALERT_MS, GAIN);
    }
}

/// Off until the store says otherwise, which is the stored default too.
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
    if let Some(value) = get_bool(port, Field::AlertSounds) {
        ENABLED.store(value, Ordering::Relaxed);
    }
}
