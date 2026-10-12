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

//! The FADT's IA-PC boot architecture flags (ACPI 6.5, 5.2.9.3, at offset
//! 109): what legacy hardware the platform says it has. A table too short
//! to hold the field (revision 1) says nothing.

const OFFSET: usize = 109;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BootArch {
    /// LPC or ISA devices the namespace may describe (bit 0).
    pub legacy_devices: bool,
    /// An 8042 keyboard controller (bit 1).
    pub has_8042: bool,
    pub no_vga: bool,
    pub no_msi: bool,
    pub no_aspm: bool,
    /// No CMOS RTC at its legacy ports (bit 5).
    pub no_cmos_rtc: bool,
}

pub fn boot_arch(fadt: &[u8]) -> Option<BootArch> {
    let b = fadt.get(OFFSET..OFFSET + 2)?;
    let v = u16::from_le_bytes([b[0], b[1]]);
    let bit = |n: u16| v & (1 << n) != 0;
    Some(BootArch {
        legacy_devices: bit(0),
        has_8042: bit(1),
        no_vga: bit(2),
        no_msi: bit(3),
        no_aspm: bit(4),
        no_cmos_rtc: bit(5),
    })
}
