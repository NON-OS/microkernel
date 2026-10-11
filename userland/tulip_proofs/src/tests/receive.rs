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

//! Frames in: the FCS cut off, only the drawn address and broadcast let
//! through, a bad frame dropped but its slot recycled, and a ring the driver
//! has emptied filling again once slots go back with a poll demand.

use super::util::{drawn, frame_to, slot, up};
use crate::bench::Bench;
use crate::model::{with_part, Delivered};

#[test]
fn a_frame_arrives_without_its_fcs_and_its_slot_goes_back() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    assert_eq!(d.rx.consume(), None, "nothing until the part lets go");
    let frame = frame_to(drawn(), 98, 0x5A);
    assert_eq!(with_part(|p| p.deliver(&frame, false)), Delivered::Taken);
    let (idx, len) = d.rx.consume().expect("a frame");
    assert_eq!((idx, len), (0, 98));
    assert_eq!(&slot(d.rx.pool_va, idx)[..98], &frame[..]);
    d.rx.recycle(idx);
    let rdra = with_part(|p| p.csr[3]);
    assert_eq!(with_part(|p| p.mem.u32(rdra)), 0x8000_0000);
}

#[test]
fn the_filter_passes_the_station_and_broadcast_only() {
    let mut bench = Bench::new();
    let _d = up(&mut bench);
    let other = [0x02, 0, 0, 0, 0, 1];
    assert_eq!(with_part(|p| p.deliver(&frame_to(other, 60, 1), false)), Delivered::Filtered);
    let multicast = [0x01, 0, 0x5E, 0, 0, 1];
    assert_eq!(with_part(|p| p.deliver(&frame_to(multicast, 60, 1), false)), Delivered::Filtered);
    assert_eq!(with_part(|p| p.deliver(&frame_to([0xFF; 6], 60, 1), false)), Delivered::Taken);
}

#[test]
fn errored_runt_and_oversized_frames_come_back_empty() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    with_part(|p| {
        p.deliver(&frame_to(drawn(), 64, 1), true);
        p.deliver(&frame_to(drawn(), 10, 2), false);
        p.deliver(&frame_to(drawn(), 1600, 3), false);
    });
    for want in 0..3u16 {
        assert_eq!(d.rx.consume(), Some((want, 0)));
        d.rx.recycle(want);
    }
}
