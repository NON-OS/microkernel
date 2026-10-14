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

//! Keeping the aux stream framed: a part packet left by a lost byte is
//! dropped after a pause rather than taking the next packet's header, and
//! a mouse that reset is noticed, not read as a right click.

use crate::mouse::event::MouseEvent;
use crate::mouse::packet::BUTTON_RIGHT;
use crate::mouse::parser::MouseParser;
use crate::mouse::ring::MouseRing;

fn feed(p: &mut MouseParser, ring: &mut MouseRing, at_ms: u64, bytes: &[u8]) {
    bytes.iter().for_each(|&b| p.absorb(b, at_ms, ring));
}

fn events(ring: &mut MouseRing) -> Vec<MouseEvent> {
    core::iter::from_fn(|| ring.pop()).collect()
}

#[test]
fn a_part_packet_older_than_half_a_second_is_dropped() {
    let (mut p, mut ring) = (MouseParser::new(false), MouseRing::new());
    feed(&mut p, &mut ring, 0, &[0x08, 0x10]);
    feed(&mut p, &mut ring, 501, &[0x09, 0x05, 0x03]);
    let ev = events(&mut ring);
    assert_eq!(ev.len(), 1);
    assert_eq!((ev[0].dx, ev[0].dy, ev[0].buttons), (5, -3, 1), "the stale half took the header");
    assert_eq!(ring.sync_errors, 1);
}

#[test]
fn a_packet_whose_bytes_come_close_together_is_kept_whole() {
    let (mut p, mut ring) = (MouseParser::new(false), MouseRing::new());
    feed(&mut p, &mut ring, 0, &[0x09]);
    feed(&mut p, &mut ring, 500, &[0x05]);
    feed(&mut p, &mut ring, 1000, &[0x03]);
    assert_eq!(events(&mut ring).len(), 1, "500 ms apart is still one packet");
    feed(&mut p, &mut ring, u64::MAX, &[0x09, 0x01]);
    feed(&mut p, &mut ring, 0, &[0x01]);
    assert_eq!(events(&mut ring).len(), 1, "a clock that went back is no pause");
}

#[test]
fn a_mouse_that_reset_is_noticed_and_never_read_as_a_right_click() {
    for wheel in [false, true] {
        let (mut p, mut ring) = (MouseParser::new(wheel), MouseRing::new());
        feed(&mut p, &mut ring, 0, &[0xAA, 0x00, 0x08, 0x01, 0x01]);
        assert!(p.take_reset(), "BAT 0xAA 0x00 went unnoticed");
        assert!(!p.take_reset(), "asking clears it");
        assert!(events(&mut ring).iter().all(|e| e.buttons & BUTTON_RIGHT == 0));
    }
}

#[test]
fn a_packet_that_merely_starts_with_0xaa_is_still_a_packet() {
    let (mut p, mut ring) = (MouseParser::new(false), MouseRing::new());
    feed(&mut p, &mut ring, 0, &[0xAA, 0x01, 0x01]);
    assert!(!p.take_reset());
    assert_eq!(events(&mut ring).len(), 1);
}
