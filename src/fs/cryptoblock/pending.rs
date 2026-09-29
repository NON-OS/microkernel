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
//! Deferred writes gathered into runs, so a large file costs one device
//! request per run instead of one per sector.
//!
//! A run is up to RUN_SECTORS sealed sectors at consecutive device LBAs.
//! It goes to the device when the next sector does not follow it, when it
//! is full, and before any flushing write or any read, so nothing can read
//! or commit past a sector still held here.

use alloc::vec::Vec;
use spin::Mutex;

use super::constants::SECTOR_BYTES;
use super::map_block::map_block_error;
use super::CryptoBlockError;

/// The most every block driver takes in one request.
pub(super) const RUN_SECTORS: usize = 64;

struct Run {
    /// Device LBA of the first sector held.
    start: u64,
    bytes: Vec<u8>,
}

static PENDING: Mutex<Run> = Mutex::new(Run { start: 0, bytes: Vec::new() });

/// Hold one sealed sector for the device LBA `at`.
pub(super) fn hold(at: u64, sector: &[u8; SECTOR_BYTES]) -> Result<(), CryptoBlockError> {
    let mut run = PENDING.lock();
    let held = (run.bytes.len() / SECTOR_BYTES) as u64;
    if held > 0 && (run.start.checked_add(held) != Some(at) || held as usize == RUN_SECTORS) {
        send(&mut run)?;
    }
    if run.bytes.is_empty() {
        run.start = at;
        run.bytes.reserve_exact(RUN_SECTORS * SECTOR_BYTES);
    }
    run.bytes.extend_from_slice(sector);
    Ok(())
}

/// Send whatever is held. Called before every read and every flush.
pub(super) fn drain() -> Result<(), CryptoBlockError> {
    send(&mut PENDING.lock())
}

fn send(run: &mut Run) -> Result<(), CryptoBlockError> {
    if run.bytes.is_empty() {
        return Ok(());
    }
    /* Held or not, a run that failed is not retried from here. */
    let sent = crate::hardware::block_device::write(run.start, &run.bytes);
    run.bytes.clear();
    sent.map_err(map_block_error)
}
