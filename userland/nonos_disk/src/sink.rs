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

//! Where the sectors go.

/// Every layout in this crate is in 512-byte sectors. A 4K-native disk still
/// presents 512-byte logical sectors to the drivers this runs over.
pub const SECTOR_SIZE: usize = 512;

/// A transfer the device refused. The value is the driver's own status,
/// carried through untouched so the receipt can show it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SinkError(pub i32);

/// A device that takes whole sectors at an address.
///
/// `write_at` and `read_at` get slices that are a non-zero multiple of
/// [`SECTOR_SIZE`] and never reach past `capacity_sectors()`; the writer
/// checks both before calling. A sink may split a slice into as many device
/// requests as its driver needs.
pub trait BlockSink {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError>;
    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError>;
    fn read_at(&mut self, lba: u64, out: &mut [u8]) -> Result<(), SinkError>;
    /// Everything written so far reaches the medium before this returns.
    fn flush(&mut self) -> Result<(), SinkError>;
}
