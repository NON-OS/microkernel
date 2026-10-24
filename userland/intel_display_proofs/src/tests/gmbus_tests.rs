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

//! EDID over DDC through GMBUS, on the board model's monitor.

use crate::engine::access::Until;
use crate::engine::error::Error;
use crate::engine::gmbus::read_block;
use crate::engine::gmbus_regs::{read_cmd, write_cmd, GMBUS0, GMBUS1, GMBUS3, SW_RDY};
use crate::model::{Gen9, Ticks};

fn monitor_on(pin: u32) -> Gen9 {
    let mut m = Gen9::new(1);
    m.ddc.monitor = Some((pin, (0..=255).collect()));
    m
}

#[test]
fn a_block_streams_four_bytes_at_a_time_then_stops() {
    let (mut m, t) = (monitor_on(5), Ticks::new());
    let mut b = [0u8; 128];
    read_block(&mut m, &t, 1, 0, &mut b, &Until::new(&t, 100_000)).unwrap();
    assert!(b.iter().enumerate().all(|(i, &x)| x == i as u8));
    read_block(&mut m, &t, 1, 1, &mut b, &Until::new(&t, 100_000)).unwrap();
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
    let (mut m, t) = (monitor_on(4), Ticks::new());
    let mut b = [0u8; 128];
    assert_eq!(read_block(&mut m, &t, 1, 0, &mut b, &Until::new(&t, 100_000)), Err(Error::Layout));
    assert!(m.writes.iter().any(|w| w.0 == GMBUS1 && w.1 == 1 << 31), "interrupt cleared");
    let no_ddc = read_block(&mut m, &t, 0, 0, &mut b, &Until::new(&t, 100_000));
    assert_eq!(no_ddc, Err(Error::Argument), "DDI A has no DDC");
}

#[test]
fn block_two_writes_the_segment_pointer_first_and_a_sink_without_it_naks() {
    let (mut m, t) = (monitor_on(5), Ticks::new());
    let mut b = [0u8; 128];
    assert_eq!(read_block(&mut m, &t, 1, 3, &mut b, &Until::new(&t, 100_000)), Err(Error::Layout));
    let at = m.writes.iter().position(|w| *w == (GMBUS1, write_cmd(0x30, 1))).unwrap();
    assert_eq!(m.writes[at - 1], (GMBUS3, 1), "segment 1, put before the command");
    assert!(!m.writes.iter().any(|w| w.0 == GMBUS1 && w.1 & 1 == 1), "no read after the NAK");
}
