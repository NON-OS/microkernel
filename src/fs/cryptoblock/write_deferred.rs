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

use core::sync::atomic::{AtomicBool, Ordering};

use super::seal::seal;
use super::window::device_lba;
use super::CryptoBlockError;

static SAID: AtomicBool = AtomicBool::new(false);

/// Seal one block and hold it for the device, unflushed. A caller writing
/// many blocks flushes once after the last, before anything points at them;
/// consecutive blocks reach the device as one request.
pub fn write_deferred(key: &[u8; 32], lba: u64, plain: &[u8]) -> Result<(), CryptoBlockError> {
    let at = device_lba(lba)?;
    let sector = seal(key, lba, plain)?;
    let held = super::pending::hold(at, &sector);
    /*
     * Advanced even when refused: a failed send may have reached the disk
     * in part.
     */
    super::epoch::advance();
    held
}

/*
 * Send every sector held for the device now, so a refusal reaches the
 * caller that sealed them, never whichever caller writes or reads next.
 */
pub fn flush_held() -> Result<(), CryptoBlockError> {
    super::pending::drain()
}

/*
 * A writer is leaving the volume, `swept` when it never runs again. What it
 * left held is dropped unsent, so no reader's drain or later writer sends a
 * sector its own caller never sealed; the count moves, since the disk may
 * hold part of what a dead writer sent.
 */
pub fn drop_held(swept: bool) {
    let dropped = super::pending_send::drop_held();
    if dropped && !SAID.swap(true, Ordering::Relaxed) {
        crate::log::error!("[CRYPTOBLOCK] a writer left sectors held; dropped unsent");
    }
    if dropped || swept {
        super::epoch::advance();
    }
}
