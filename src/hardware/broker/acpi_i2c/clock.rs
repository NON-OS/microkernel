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

/// Source clock for a platform (ACPI-enumerated) DesignWare I2C controller,
/// keyed off its `_HID`, as Linux acpi_apd and acpi_lpss fix them: AMD
/// Carrizo-era AMD0010 runs the block at 133 MHz, AMDI0010 and AMDI0510
/// (Ryzen) at 150 MHz, the Intel LPSS platform devices at 100 MHz. A clock
/// assumed higher than the real one only slows the bus, so the AMD default
/// for an unknown AMD id is the higher figure.
pub(super) fn source_clock_hz(hid: &[u8; 8]) -> u32 {
    if &hid[..7] == b"AMD0010" && hid[7] == 0 {
        133_000_000
    } else if &hid[..3] == b"AMD" {
        150_000_000
    } else {
        100_000_000
    }
}
