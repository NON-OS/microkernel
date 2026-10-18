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

//! Frames in: the FCS cut off, a bad frame dropped but its slot recycled,
//! and a ring the driver has emptied filling again once slots go back.

use super::util::{slot, up};
use crate::bench::Bench;
use crate::model::{with_part, Delivered};

#[test]
fn a_frame_arrives_without_its_fcs_and_its_slot_goes_back() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    assert_eq!(d.rx.consume(), None, "nothing until the part lets go");
    let frame: Vec<u8> = (0..98u8).collect();
    assert_eq!(with_part(|p| p.deliver(&frame, false)), Delivered::Taken);
    let (idx, len) = d.rx.consume().expect("a frame");
    assert_eq!((idx, len), (0, 98));
    assert_eq!(&slot(d.rx.pool_va, idx)[..98], &frame[..]);
    d.rx.recycle(idx);
    let rdra = with_part(|p| p.rings.rdra);
    assert_eq!(with_part(|p| (p.mem.u16(rdra + 6), p.mem.u32(rdra + 8))), (0x8000, 0));
}

#[test]
fn errored_runt_and_oversized_frames_come_back_empty() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    with_part(|p| {
        p.deliver(&[1; 64], true);
        p.deliver(&[2; 10], false);
        p.deliver(&[3; 1600], false);
    });
    for want in 0..3u16 {
        assert_eq!(d.rx.consume(), Some((want, 0)));
        d.rx.recycle(want);
    }
}

#[test]
fn a_full_ring_misses_frames_until_slots_go_back() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for n in 0..32u8 {
        assert_eq!(with_part(|p| p.deliver(&[n; 60], false)), Delivered::Taken);
    }
    assert_eq!(with_part(|p| p.deliver(&[0xFF; 60], false)), Delivered::NoBuffer);
    for n in 0..32u16 {
        let (idx, len) = d.rx.consume().expect("frame");
        assert_eq!((idx, len, slot(d.rx.pool_va, idx)[0]), (n, 60, n as u8));
        d.rx.recycle(idx);
    }
    assert_eq!(with_part(|p| p.deliver(&[7; 60], false)), Delivered::Taken);
    assert_eq!(d.rx.consume(), Some((0, 60)), "the walk wrapped with the part");
}
