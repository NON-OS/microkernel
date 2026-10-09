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

//! Where on the disk a sealed volume lies.
//!
//! A volume is a window of the kernel's block device: its LBA 0 is the
//! window's base. Every block is sealed to its LBA inside the window, so a
//! volume copied to another base still opens, and a block moved to another
//! place in it does not. Nothing is read or written before a window is set,
//! and nothing outside it ever is.

use core::sync::atomic::{AtomicU64, Ordering};

use super::CryptoBlockError;

static BASE: AtomicU64 = AtomicU64::new(0);
/// Sectors in the window; zero while none is set.
static SECTORS: AtomicU64 = AtomicU64::new(0);

/// Open the window `[base, base + sectors)` of the device.
pub fn set_window(base: u64, sectors: u64) -> Result<(), CryptoBlockError> {
    if sectors == 0 || base.checked_add(sectors).is_none() {
        return Err(CryptoBlockError::OutOfRange);
    }
    SECTORS.store(0, Ordering::SeqCst);
    BASE.store(base, Ordering::SeqCst);
    SECTORS.store(sectors, Ordering::SeqCst);
    super::epoch::advance();
    Ok(())
}

/// Sectors in the window, if one is set.
pub fn window_sectors() -> Option<u64> {
    match SECTORS.load(Ordering::SeqCst) {
        0 => None,
        n => Some(n),
    }
}

/// The device LBA of the volume's `lba`, or why there is none.
pub(super) fn device_lba(lba: u64) -> Result<u64, CryptoBlockError> {
    let sectors = window_sectors().ok_or(CryptoBlockError::NoWindow)?;
    if lba >= sectors {
        return Err(CryptoBlockError::OutOfRange);
    }
    Ok(BASE.load(Ordering::SeqCst) + lba)
}
