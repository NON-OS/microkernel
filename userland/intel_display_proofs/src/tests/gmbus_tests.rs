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

//! EDID over DDC through GMBUS.

use crate::engine::error::Error;
use crate::engine::gmbus::read_edid;
use crate::engine::gmbus_regs::{read_cmd, GMBUS0, GMBUS1, SW_RDY};
use crate::model::{Gen9, Ticks};

fn monitor_on(pin: u32) -> Gen9 {
    let mut m = Gen9::new(1);
    m.ddc.monitor = Some((pin, (0..=255).collect()));
    m
}

#[test]
fn a_block_streams_four_bytes_at_a_time_then_stops() {
    let mut m = monitor_on(5);
    let mut b = [0u8; 128];
    read_edid(&mut m, &Ticks::new(), 1, 0, &mut b).unwrap();
    assert!(b.iter().enumerate().all(|(i, &x)| x == i as u8));
    let mut b = [0u8; 128];
    read_edid(&mut m, &Ticks::new(), 1, 128, &mut b).unwrap();
    assert_eq!((b[0], b[127]), (128, 255));
    assert_eq!(m.get(GMBUS0), 0, "pins released");
    assert!(!m.ddc.active, "a stop ended the transaction");
}

#[test]
fn the_command_names_address_index_length_and_direction() {
    let c = read_cmd(0x50, 0x80, 128);
    assert_eq!(c & 0xFF, (0x50 << 1) | 1);
    assert_eq!((c >> 8) & 0xFF, 0x80);
    assert_eq!((c >> 16) & 0x1FF, 128);
    assert_ne!(c & SW_RDY, 0);
}

#[test]
fn nothing_on_the_pins_is_a_nak_and_the_controller_is_cleared() {
    let mut m = monitor_on(4);
    let mut b = [0u8; 16];
    assert_eq!(read_edid(&mut m, &Ticks::new(), 1, 0, &mut b), Err(Error::Layout));
    assert!(m.writes.iter().any(|w| w.0 == GMBUS1 && w.1 == 1 << 31), "interrupt cleared");
    assert_eq!(
        read_edid(&mut m, &Ticks::new(), 0, 0, &mut b),
        Err(Error::Argument),
        "DDI A has no DDC"
    );
    assert_eq!(read_edid(&mut m, &Ticks::new(), 2, 0, &mut [0u8; 257]), Err(Error::Argument));
}
