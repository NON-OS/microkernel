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

//! Frames out: padded to the wire minimum, sent whole and in order across
//! many generation flips, slots reclaimed from completions, and a ring the
//! device has not completed is full.

use super::util::{slot, up};
use crate::bench::Bench;
use crate::model::with_device;
use crate::setup::Driver;
use crate::tx_stage::stage;

fn post(d: &mut Driver, frame: &[u8], ring: bool) {
    d.tx.reclaim();
    assert!(!d.tx.full());
    let len = stage(slot(d.tx.pool_va, d.tx.fill), frame);
    let next = d.tx.post(len as u16);
    if ring {
        d.regs.tx_prod(next);
    }
}

#[test]
fn frames_go_out_in_order_across_the_generation_flips() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for n in 0..100u8 {
        let frame = vec![n; 14 + n as usize];
        post(&mut d, &frame, true);
        let mut want = frame.clone();
        want.resize(want.len().max(60), 0);
        assert_eq!(with_device(|dev| dev.sent.pop()), Some(want));
    }
    d.tx.reclaim();
    assert_eq!(d.tx.clean, d.tx.fill);
}

#[test]
fn a_ring_the_device_has_not_completed_is_full_until_it_does() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for _ in 0..31 {
        post(&mut d, &[9; 60], false);
    }
    d.tx.reclaim();
    assert!(d.tx.full(), "one slot always stays empty");
    d.regs.tx_prod(d.tx.fill as u32);
    assert_eq!(with_device(|dev| dev.sent.len()), 31);
    d.tx.reclaim();
    assert!(!d.tx.full() && d.tx.clean == d.tx.fill);
}
