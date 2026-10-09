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

use crate::hardware::block_device::{self, BlockDeviceError};

pub(super) fn read(lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
    if super::ram::on() {
        return super::ram::read(lba, out);
    }
    block_device::read(lba, out)
}

pub(super) fn write(lba: u64, bytes: &[u8]) -> Result<(), BlockDeviceError> {
    if super::ram::on() {
        return super::ram::write(lba, bytes);
    }
    block_device::write(lba, bytes)
}

/// RAM keeps what it is given at once; only a disk has a cache to empty.
pub(super) fn flush() -> Result<(), BlockDeviceError> {
    if super::ram::on() {
        return Ok(());
    }
    block_device::flush()
}
