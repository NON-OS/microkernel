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

//! The data volume's links to the disk and to zero-state: whether the boot
//! keeps (off until a test arms it, as policy would), the volume
//! scope the block clients' gate reads, and the held sectors' flush, which
//! has nothing to send with no disk under it.

use std::sync::atomic::{AtomicBool, Ordering};

use crate::hardware::block_device::BlockDeviceError;

static KEEPING: AtomicBool = AtomicBool::new(false);

pub fn keeping() -> bool {
    KEEPING.load(Ordering::Relaxed)
}

/// Arms or disarms keeping for the test that holds the turn.
pub fn set_keeping(on: bool) {
    KEEPING.store(on, Ordering::Relaxed);
}

pub fn volume_scope() -> Result<(), BlockDeviceError> {
    Ok(())
}

pub fn flush_held() -> Result<(), crate::fs::cryptoblock::CryptoBlockError> {
    Ok(())
}
