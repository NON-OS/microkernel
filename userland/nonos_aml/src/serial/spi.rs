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

//! SpiSerialBusV2's type-specific part (ACPI 6.5, 6.4.3.8.2.2; offsets and
//! values recalled, unverified): flags bit 0 the wire mode (three-wire when
//! set), bit 1 the chip select's polarity (active high when set), the rest
//! reserved; then the connection speed u32, data bit length, clock phase
//! (0 first edge, 1 second), clock polarity (0 idles low, 1 high) and the
//! chip-select line u16. A descriptor with a reserved flag set, a phase or
//! polarity past 1, no data bits or no speed is refused, not guessed at.

use super::bus::Bus;
use super::le::{u16_at, u32_at};

pub const SPI_DATA: usize = 9;
const KNOWN_FLAGS: u16 = 0b11;

/// `d` is the type-specific data, at least `SPI_DATA` long.
pub fn spi(d: &[u8], flags: u16) -> Option<Bus> {
    let d = d.get(..SPI_DATA)?;
    let (speed_hz, data_bits, phase, polarity) = (u32_at(d, 0), d[4], d[5], d[6]);
    let reserved = flags & !KNOWN_FLAGS != 0 || phase > 1 || polarity > 1;
    if reserved || data_bits == 0 || speed_hz == 0 {
        return None;
    }
    Some(Bus::Spi {
        speed_hz,
        data_bits,
        phase,
        polarity,
        chip_select: u16_at(d, 7),
        three_wire: flags & 1 != 0,
        select_active_high: flags & 2 != 0,
    })
}
