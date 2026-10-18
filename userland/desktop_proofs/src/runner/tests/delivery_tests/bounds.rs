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

//! Frames at the boundaries, and the event decode's own bound.

use super::super::super::dispatch::{parse_delivery, DELIVERY_LEN};
use super::frames::delivery;
use crate::input::{InputEvent, InputKind};

#[test]
fn boundary_frames() {
    assert!(parse_delivery(&[]).is_none());
    let f = delivery(0, 0x41, 1, 2);
    assert!(parse_delivery(&f[..DELIVERY_LEN - 1]).is_none(), "one byte short");
    assert!(parse_delivery(&f).is_some());
    for kind in 0..=8u16 {
        let ev = parse_delivery(&delivery(kind, 0, 0, 0));
        assert_eq!(ev.is_some(), kind <= 7, "kind {kind}");
    }
    let ev = parse_delivery(&delivery(4, u32::MAX, i32::MIN, i32::MAX));
    assert!(matches!(ev.map(|e| e.kind), Some(InputKind::Wheel)));
    let mut bad = f.clone();
    bad[0] ^= 1;
    assert!(parse_delivery(&bad).is_none(), "the wrong magic");
}

/// The event decode on its own, as the Linux guest's wayland input and the
/// installer call it after their own header checks: it reads only a whole
/// 32-byte event, whatever it is handed.
#[test]
fn the_event_decode_reads_only_a_whole_event() {
    for n in 0..48 {
        let p = vec![0u8; n];
        assert_eq!(InputEvent::from_delivery(&p).is_some(), n >= 32, "{n} bytes");
    }
}
