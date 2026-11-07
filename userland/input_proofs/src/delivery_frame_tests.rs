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

//! The router's delivery frame is the layout abi/wire.toml pins as
//! [input.delivery_v1], and what nonos_libc::decode_delivery reads back:
//! the event exactly, the window when the flags carry one, and a frame cut
//! to its first 40 bytes still read as the event alone.

use nonos_libc::InputEvent;

use crate::libc_frame::delivery::{
    decode_delivery, delivery_window, DELIVERY_FLAG_WINDOW, DELIVERY_LEN, DELIVERY_LEN_V1,
    DELIVERY_WINDOW_OFFSET,
};
use crate::protocol::{encode_delivery, DELIVERY_LEN as ROUTER_LEN};

const WIRE: &str = include_str!("../../../abi/wire.toml");

/// The value of `key` in table `table`: a number, or the offset of a field.
fn pinned(table: &str, key: &str) -> usize {
    let body = WIRE.split(&format!("[{table}]\n")).nth(1).expect(table);
    let body = body.split("\n[").next().expect(table);
    let line = body.lines().find(|l| l.split('=').next().map(str::trim) == Some(key));
    let v = line.expect(key).split_once('=').expect(key).1;
    let v = v.split("offset =").nth(1).unwrap_or(v);
    let v = v.trim().trim_start_matches('{').split([',', '}']).next().expect(key);
    v.trim().parse().expect(key)
}

fn event() -> InputEvent {
    InputEvent {
        kind: 5,
        flags: 0x0042,
        code: 0x1111_2222,
        x: -7,
        y: 1_000_000,
        delta_x: -3,
        delta_y: 9,
        timestamp_ns: 0x0102_0304_0506_0708,
    }
}

fn same(a: &InputEvent, b: &InputEvent) -> bool {
    (a.kind, a.flags, a.code, a.x, a.y, a.delta_x, a.delta_y, a.timestamp_ns)
        == (b.kind, b.flags, b.code, b.x, b.y, b.delta_x, b.delta_y, b.timestamp_ns)
}

#[test]
fn the_frame_is_the_pinned_layout() {
    let t = "input.delivery_v1";
    assert_eq!(pinned(t, "size_bytes"), ROUTER_LEN);
    assert_eq!(ROUTER_LEN, DELIVERY_LEN);
    assert_eq!(pinned(t, "window_id"), DELIVERY_WINDOW_OFFSET);
    assert_eq!(pinned(t, "event"), 8);
    assert_eq!(pinned(t, "event") + pinned("input.event_v1", "size_bytes"), DELIVERY_LEN_V1);
    assert_eq!(pinned("input.delivery_v1.flags", "window"), DELIVERY_FLAG_WINDOW as usize);
    let mut f = [0u8; ROUTER_LEN];
    encode_delivery(&mut f, &event(), Some(0xAABB_CCDD));
    let at = |k: &str| pinned(t, k);
    assert_eq!(&f[at("magic")..at("magic") + 4], b"PNIN");
    assert_eq!(u16::from_le_bytes([f[at("version")], f[at("version") + 1]]), 1);
    assert_eq!(u16::from_le_bytes([f[at("flags")], f[at("flags") + 1]]), DELIVERY_FLAG_WINDOW);
    let w = at("window_id");
    assert_eq!(u32::from_le_bytes([f[w], f[w + 1], f[w + 2], f[w + 3]]), 0xAABB_CCDD);
    assert_eq!(&f[at("reserved")..at("reserved") + 4], &[0; 4]);
}

#[test]
fn a_routed_frame_reads_back_with_its_window() {
    let mut f = [0u8; ROUTER_LEN];
    encode_delivery(&mut f, &event(), Some(7));
    let d = decode_delivery(&f).expect("a frame");
    assert!(same(&d.event, &event()));
    assert_eq!(d.window_id, Some(7));
    assert_eq!(delivery_window(&f), Some(7));
}

#[test]
fn a_frame_for_no_window_reads_back_without_one() {
    let mut f = [0u8; ROUTER_LEN];
    encode_delivery(&mut f, &event(), None);
    let d = decode_delivery(&f).expect("a frame");
    assert!(same(&d.event, &event()));
    assert_eq!(d.window_id, None);
    assert_eq!(&f[40..48], &[0; 8]);
}

#[test]
fn a_receiver_reading_forty_bytes_reads_the_event_as_before() {
    // The kernel hands a receiver as much of a message as its buffer holds;
    // one with a 40-byte buffer gets the frame's first 40 bytes.
    let mut f = [0u8; ROUTER_LEN];
    encode_delivery(&mut f, &event(), Some(9));
    let d = decode_delivery(&f[..DELIVERY_LEN_V1]).expect("a v1 read");
    assert!(same(&d.event, &event()));
    assert_eq!(d.window_id, None, "no window past the end of what was read");
    assert!(decode_delivery(&f[..DELIVERY_LEN_V1 - 1]).is_none());
}

#[test]
fn a_frame_of_another_kind_is_not_read() {
    let mut f = [0u8; ROUTER_LEN];
    encode_delivery(&mut f, &event(), Some(1));
    let mut other = f;
    other[0] ^= 1;
    assert!(decode_delivery(&other).is_none());
    let mut later = f;
    later[4] = 2;
    assert!(decode_delivery(&later).is_none());
}
