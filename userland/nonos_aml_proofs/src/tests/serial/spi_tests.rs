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

//! SpiSerialBusV2: a descriptor read field by field, and one that is cut
//! short or carries a reserved value refused rather than guessed at.

use super::build::serial;
use crate::serial::{serial_buses, spi, Bus};

fn end(t: Vec<u8>) -> Vec<u8> {
    [t, vec![0x79, 0]].concat()
}

/// 8 MHz, 8 bits, phase 0, polarity 1, chip select 1.
const DATA: [u8; 9] = [0x00, 0x12, 0x7A, 0x00, 8, 0, 1, 1, 0];

#[test]
fn a_descriptor_decodes() {
    let t = end(serial(2, 0b01, &DATA, "\\_SB.SPI1"));
    let c = &serial_buses(&t)[0];
    let want = Bus::Spi {
        speed_hz: 8_000_000,
        data_bits: 8,
        phase: 0,
        polarity: 1,
        chip_select: 1,
        three_wire: true,
        select_active_high: false,
    };
    assert_eq!((c.bus.clone(), c.source.as_slice()), (want, &b"\\_SB.SPI1"[..]));
}

#[test]
fn reserved_values_are_refused() {
    let with = |i: usize, v: u8| {
        let mut d = DATA;
        d[i] = v;
        d
    };
    assert_eq!(spi(&with(5, 2), 0), None, "phase 2");
    assert_eq!(spi(&with(6, 2), 0), None, "polarity 2");
    assert_eq!(spi(&with(4, 0), 0), None, "no data bits");
    assert_eq!(spi(&[0, 0, 0, 0, 8, 0, 0, 0, 0], 0), None, "no speed");
    assert_eq!(spi(&DATA, 0b100), None, "a reserved type flag");
    assert!(spi(&DATA, 0b11).is_some());
    assert!(serial_buses(&end(serial(2, 0b1000, &DATA, "S"))).is_empty());
}

#[test]
fn a_cut_descriptor_is_refused() {
    assert_eq!(spi(&DATA[..8], 0), None);
    let full = end(serial(2, 0, &DATA, "\\_SB.SPI1"));
    // Claim nine bytes of data but end the descriptor inside them.
    let mut cut = serial(2, 0, &DATA[..5], "");
    let at = cut.len() - 5 - 1 - 2;
    cut[at] = 9;
    assert!(serial_buses(&end(cut)).is_empty());
    assert_eq!(serial_buses(&full).len(), 1);
}
