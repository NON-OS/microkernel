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

//! Where the volume's sealed sectors go: the disk, or RAM on a live boot.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::hardware::block_device::{self, BlockDeviceError};
use crate::security::zerostate::volume_rule::may_write;

static REFUSED: AtomicBool = AtomicBool::new(false);

pub(super) fn read(lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    let _io = super::volume_scope()?;
    if super::ram::on() {
        return super::ram::read(lba, out);
    }
    block_device::read(lba, out)
}

/*
 * A disk takes a sealed sector only on a boot that keeps (volume_rule.rs).
 * The volume is opened in RAM until then, so a refusal here is a bug in
 * that switch failing closed, said once.
 */
pub(super) fn write(lba: u64, bytes: &[u8]) -> Result<(), BlockDeviceError> {
    let _io = super::volume_scope()?;
    let ram = super::ram::on();
    if !may_write(ram, crate::security::zerostate::keeping()) {
        if !REFUSED.swap(true, Ordering::Relaxed) {
            crate::sys::serial::print(b"[DATA] volume write refused: this boot keeps nothing\n");
        }
        return Err(BlockDeviceError::AccessDenied);
    }
    if ram {
        return super::ram::write(lba, bytes);
    }
    block_device::write(lba, bytes)
}

/// RAM keeps what it is given at once; only a disk has a cache to empty.
pub(super) fn flush() -> Result<(), BlockDeviceError> {
    let _io = super::volume_scope()?;
    if super::ram::on() {
        return Ok(());
    }
    if !may_write(false, crate::security::zerostate::keeping()) {
        return Err(BlockDeviceError::AccessDenied);
    }
    block_device::flush()
}
