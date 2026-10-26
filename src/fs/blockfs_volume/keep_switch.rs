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

//! An installed disk's volume on a boot that keeps nothing yet: held in RAM
//! as a live stick's is, and dropped, not copied, when keeping turns on
//! (zerostate/volume_rule.rs). The disk's own volume is opened after that.

use alloc::format;
use core::sync::atomic::{AtomicBool, Ordering};

use super::super::error::VolumeError;
use super::super::import_feed::with_streams;
use super::super::list::names_in;
use super::super::say::say;
use super::super::state::VOLUME;
use super::open_session_volume;
use crate::security::zerostate::keeping;
use crate::security::zerostate::volume_rule::{drop_at_arm, place, Place};

/// The open volume is in RAM in place of an installed disk's. Set and
/// cleared only under the volume's lock.
pub(in super::super) static HELD: AtomicBool = AtomicBool::new(false);

/// None when the volume opens on the disk; else the RAM volume's opening.
pub(in super::super) fn in_memory(live: bool) -> Option<Result<(), VolumeError>> {
    if place(!live, keeping()) == Place::Disk {
        return None;
    }
    let opened = open_session_volume(!live);
    /* Armed meanwhile: whichever of the arm and this sees HELD drops it. */
    if !live && keeping() {
        keeping_on();
        return None;
    }
    Some(opened)
}

/// Keeping turned on. The stream lock is held over the switch, so no import
/// is part way through it, and a stream into the dropped volume ends.
pub fn keeping_on() {
    with_streams(|| ((), drop_held()))
}

fn drop_held() -> bool {
    let mut volume = crate::smp::write_responsive(&VOLUME);
    if !drop_at_arm(HELD.swap(false, Ordering::AcqRel)) {
        return false;
    }
    let names = volume.as_ref().and_then(|v| names_in(v, b"").ok()).unwrap_or(0);
    // Sealed sectors still held go to RAM, never to the disk's volume.
    let _ = crate::fs::cryptoblock::flush_held();
    if let Some(mut state) = volume.take() {
        crate::crypto::secure_zero(&mut state.key);
    }
    crate::fs::cryptoblock::ram::stop();
    say(&format!(
        "[DATA] keeping turned on: the volume held in memory is dropped; {} names in it were not kept",
        names
    ));
    true
}
