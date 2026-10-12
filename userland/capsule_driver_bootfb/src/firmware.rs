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

//! The broker as bootfb uses it, a trait so the bring-up is proved on a
//! model of the kernel's record; the binary's is libc.

use super::geometry::INFO_LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Why {
    /// Not the record the kernel describes.
    Record,
    Announce,
    Claim,
    Geometry,
    /// Three-byte pixels: the console keeps the screen.
    Format,
    Map,
    /// A mode the mapped range does not hold.
    Shape,
}

pub trait Firmware {
    fn announce(&mut self) -> Result<(), Why>;
    /// The claim's epoch.
    fn claim(&mut self, device_id: u64) -> Result<u64, Why>;
    fn geometry(&mut self, device_id: u64) -> Result<[u8; INFO_LEN], Why>;
    /// Bar 0, write-combined where the kernel's own mapping is.
    fn map(&mut self, device_id: u64, epoch: u64, size: u64) -> Result<*mut u8, Why>;
    fn release(&mut self, device_id: u64);
    fn say(&mut self, line: &str);
}
