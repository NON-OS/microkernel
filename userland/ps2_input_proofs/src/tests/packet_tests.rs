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

//! The aux byte stream is the device's word: packets assemble only from a
//! byte that can open one, malformed packets are counted and dropped, an
//! overflowed axis is clamped, and the queue drops rather than overwrites.

use crate::mouse::packet::{parse, BUTTON_LEFT, FLAG_X_OVERFLOW, FLAG_Y_OVERFLOW};
use crate::mouse::parser::MouseParser;
use crate::mouse::ring::{MouseRing, MOUSE_RING_CAPACITY};

#[test]
fn a_first_byte_without_the_always_one_bit_never_opens_a_packet() {
    let mut p = MouseParser::new(false);
    let mut ring = MouseRing::new();
    for b in [0x00u8, 0x10, 0x07, 0xF7] {
        p.absorb(b, &mut ring);
    }
    assert_eq!(ring.sync_errors, 4, "every stray byte is counted");
    assert!(ring.pop().is_none(), "no packet assembled from stray bytes");
    // The next byte that can open a packet does, and the stream recovers.
    for b in [0x09u8, 0x05, 0x03] {
        p.absorb(b, &mut ring);
    }
    let ev = ring.pop().expect("resynchronised");
    assert_eq!((ev.dx, ev.dy, ev.buttons), (5, -3, BUTTON_LEFT));
}

#[test]
fn parse_is_total_over_every_short_and_hostile_packet() {
    assert!(parse(&[]).is_none());
    assert!(parse(&[0x08]).is_none());
    assert!(parse(&[0x08, 0x01]).is_none());
    for b0 in 0..=255u8 {
        for b in [0x00u8, 0x7F, 0x80, 0xFF] {
            let got = parse(&[b0, b, b, b]);
            assert_eq!(got.is_some(), b0 & 0x08 != 0, "byte0 {b0:#x}");
            if let Some(ev) = got {
                assert!((-256..=255).contains(&ev.dx) && (-255..=256).contains(&ev.dy));
            }
        }
    }
}

#[test]
fn an_overflowed_axis_is_clamped_in_its_direction_not_taken_verbatim() {
    let ev = parse(&[0x08 | 0x40 | 0x80 | 0x10, 0x01, 0x01]).expect("valid");
    assert_eq!(ev.dx, -255, "x overflow with sign set is a full step left");
    assert_eq!(ev.dy, -255, "y overflow without sign is a full step up (y is inverted)");
    assert_eq!(ev.flags, FLAG_X_OVERFLOW | FLAG_Y_OVERFLOW);
}

#[test]
fn a_wheel_mouse_takes_the_fourth_byte_as_a_signed_step() {
    let mut p = MouseParser::new(true);
    let mut ring = MouseRing::new();
    for b in [0x08u8, 0x00, 0x00, 0xFF] {
        p.absorb(b, &mut ring);
    }
    assert_eq!(ring.pop().map(|e| e.dz), Some(-1));
}

#[test]
fn a_full_queue_drops_new_packets_and_counts_them() {
    let mut p = MouseParser::new(false);
    let mut ring = MouseRing::new();
    let total = MOUSE_RING_CAPACITY + 10;
    for _ in 0..total {
        for b in [0x08u8, 0x01, 0x01] {
            p.absorb(b, &mut ring);
        }
    }
    assert_eq!(ring.events_seen as usize, total);
    assert_eq!(ring.queued(), MOUSE_RING_CAPACITY - 1, "one slot always stays empty");
    assert_eq!(ring.events_dropped as usize, total - (MOUSE_RING_CAPACITY - 1));
}
