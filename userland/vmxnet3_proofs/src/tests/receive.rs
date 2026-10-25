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

//! Frames in: whole frames come back with their slot, dropped ones free
//! their slot, a forged completion is ignored, and the queue keeps running
//! across many generation flips.

use super::util::{slot, up};
use crate::bench::Bench;
use crate::model::{with_device, Delivered};
use crate::queue::Completion;

#[test]
fn frames_arrive_in_order_across_the_generation_flips() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    assert_eq!(d.rx.consume(), None);
    for n in 0..200u32 {
        let frame = vec![n as u8; 60 + (n as usize % 40)];
        assert_eq!(with_device(|dev| dev.deliver(&frame, false)), Delivered::Taken, "frame {n}");
        let Some(Completion::Frame { idx, len }) = d.rx.consume() else { panic!("frame {n}") };
        assert_eq!((idx as u32, len as usize), (n % 32, frame.len()));
        assert_eq!(&slot(d.rx.pool_va, idx)[..frame.len()], &frame[..]);
        d.rx.post();
    }
}

#[test]
fn an_errored_or_oversized_frame_is_dropped_and_its_slot_comes_back() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    with_device(|dev| {
        dev.deliver(&[1; 64], true);
        dev.deliver(&[2; 4000], false);
        dev.deliver(&[3; 10], false);
    });
    for _ in 0..3 {
        assert_eq!(d.rx.consume(), Some(Completion::Dropped));
        d.rx.post();
    }
    // 31 handed over at bring-up, three used and three given back.
    for n in 0..31u8 {
        assert_eq!(with_device(|dev| dev.deliver(&[n; 60], false)), Delivered::Taken);
    }
    assert_eq!(with_device(|dev| dev.deliver(&[0; 60], false)), Delivered::NoBuffer);
}

// A completion for a slot the device should not have filled next, or for
// ring 1 which was never given a buffer, is passed over without a refill.
#[test]
fn a_completion_for_the_wrong_slot_is_stray() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    let fill = d.rx.fill;
    with_device(|dev| dev.complete(5, 0, 60));
    assert_eq!(d.rx.consume(), Some(Completion::Stray));
    with_device(|dev| dev.complete(0, 1, 60));
    assert_eq!(d.rx.consume(), Some(Completion::Stray));
    assert_eq!(d.rx.fill, fill, "nothing was handed back");
}
