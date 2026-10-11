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

//! Frames out: padded to the wire minimum, sent whole, slots reclaimed once
//! the part lets go, and a ring with no free slot says so.

use super::util::{slot, up};
use crate::bench::Bench;
use crate::constants::csr::{CSR0, CSR0_TDMD};
use crate::model::with_part;
use crate::setup::Driver;
use crate::tx_stage::stage;

fn post(d: &mut Driver, frame: &[u8]) {
    d.tx.reclaim();
    assert!(!d.tx.full());
    let len = stage(slot(d.tx.pool_va, d.tx.tail), frame);
    d.tx.post(len as u16);
}

#[test]
fn a_short_frame_is_padded_with_zeros_not_old_bytes() {
    let mut s = [0xAAu8; 1536];
    assert_eq!(stage(&mut s, &[1; 42]), 60);
    assert!(s[..42].iter().all(|&b| b == 1) && s[42..60].iter().all(|&b| b == 0));
    assert_eq!(stage(&mut s, &[2; 1514]), 1514);
}

#[test]
fn frames_go_out_in_order_across_the_wrap() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for n in 0..100u8 {
        let frame = vec![n; 64 + n as usize];
        post(&mut d, &frame);
        d.regs.set_csr(CSR0, CSR0_TDMD);
        assert_eq!(with_part(|p| p.sent.pop()), Some(frame));
    }
    d.tx.reclaim();
    assert_eq!(d.tx.clean, d.tx.tail);
}

#[test]
fn a_ring_the_part_still_owns_is_full_until_it_lets_go() {
    let mut bench = Bench::new();
    let mut d = up(&mut bench);
    for _ in 0..31 {
        post(&mut d, &[9; 60]);
    }
    d.tx.reclaim();
    assert!(d.tx.full(), "one slot always stays empty");
    d.regs.set_csr(CSR0, CSR0_TDMD);
    assert_eq!(with_part(|p| p.sent.len()), 31);
    d.tx.reclaim();
    assert!(!d.tx.full() && d.tx.clean == d.tx.tail);
}
