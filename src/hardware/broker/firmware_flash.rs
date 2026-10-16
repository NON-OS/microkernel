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
//! The firmware's flash is never a capsule's. The PCH's SPI flash
//! controller (bus 0, device 31, function 5) programs, erases and reads
//! the firmware image, and the flash's decode window just below 4 GiB is
//! the image itself (the PCH datasheets' SPI and BIOS-range chapters, and
//! AMD's FCH ROM range, as recalled, unverified). Neither is claimed and
//! neither is mapped, whatever id the device carries. Pure, for the proofs.

/// The top 16 MiB below 4 GiB: the largest BIOS decode window, as recalled,
/// unverified; refusing all of it is safe, as no device a capsule drives
/// lives there.
pub const FLASH_WINDOW: (u64, u64) = (0xFF00_0000, 0x1_0000_0000);
pub const FLASH_FUNCTION: (u16, u8, u8, u8) = (0, 0, 31, 5);

/// Whether the PCI function at `segment`, `bus`, `device`, `function` is
/// where a PCH keeps its flash controller.
pub fn is_flash_function(segment: u16, bus: u8, device: u8, function: u8) -> bool {
    (segment, bus, device, function) == FLASH_FUNCTION
}

/// Whether `size` bytes from `base` touch the flash window.
pub fn overlaps_flash(base: u64, size: u64) -> bool {
    let end = base.saturating_add(size);
    size != 0 && base < FLASH_WINDOW.1 && end > FLASH_WINDOW.0
}
