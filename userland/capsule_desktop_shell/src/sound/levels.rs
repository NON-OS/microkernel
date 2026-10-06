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

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use nonos_policy_client::{get_bool, get_u8};
use nonos_policy_proto::Field;

// Gain at Volume 100. The stored default, 64, gives 0x2000, the level the
// shell's tones were tuned at.
const FULL_GAIN: u32 = 12_800;

static SOUND: AtomicBool = AtomicBool::new(true);
static ALERTS: AtomicBool = AtomicBool::new(false);
static VOLUME: AtomicU32 = AtomicU32::new(64);

// Read the three sound settings; a field the store does not answer keeps its last value.
pub(super) fn follow(port: u32) {
    if let Some(v) = get_bool(port, Field::SoundEnabled) {
        SOUND.store(v, Ordering::Relaxed);
    }
    if let Some(v) = get_bool(port, Field::AlertSounds) {
        ALERTS.store(v, Ordering::Relaxed);
    }
    if let Some(v) = get_u8(port, Field::Volume) {
        VOLUME.store(v.min(100) as u32, Ordering::Relaxed);
    }
}

pub(super) fn alerts_on() -> bool {
    ALERTS.load(Ordering::Relaxed)
}

// The gain to play at, or None when sound is off or the volume is zero.
pub(super) fn gain() -> Option<u16> {
    if !SOUND.load(Ordering::Relaxed) {
        return None;
    }
    let g = FULL_GAIN * VOLUME.load(Ordering::Relaxed) / 100;
    (g > 0).then_some(g as u16)
}
