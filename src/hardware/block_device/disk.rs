// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! One row of the block layer's table: a driver's disk as the layer asks it.
//! A row that stands for many disks (the block wire's endpoints and units)
//! tells them apart by the key each `Backend` carries.

use alloc::string::String;
use core::sync::atomic::AtomicU64;

use super::BlockDeviceError;

pub(super) struct Disk {
    /// What the probe log calls it: "NVMe (driver.nvme0)".
    pub shown: fn(u32) -> &'static str,
    /// What the line naming the kept disk calls it.
    pub announced: fn(u32) -> &'static str,
    /// Its size in 512-byte sectors.
    pub capacity: fn(u32) -> Result<u64, BlockDeviceError>,
    pub read: fn(u32, u64, &mut [u8]) -> Result<(), BlockDeviceError>,
    pub write: fn(u32, u64, &[u8]) -> Result<(), BlockDeviceError>,
    pub flush: fn(u32) -> Result<(), BlockDeviceError>,
    /// False for a disk 512-byte sectors cannot address; the row says why.
    pub fits: fn(u32) -> bool,
    /// The hash of the last probe line said for it.
    pub said: fn(u32) -> &'static AtomicU64,
    /// The loader's copy of the boot disk goes on answering reads once this
    /// disk is kept: a live stick, which the copy was made from.
    pub copy_answers: bool,
    /// A line of the driver's own the layer says for it, if it has one.
    pub report: Option<fn() -> Option<String>>,
}
