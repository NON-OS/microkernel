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

//! The bounds themselves: tries, time and bytes, against a sink that
//! declares too much, holds the bus, holds the clock, or offers more.

use super::kit::{honest, read, EDID_US};
use super::sink::{Fault, Sink};
use crate::engine::gmbus_xfer::deadline::XFER_US;
use crate::model::Ticks;

#[test]
fn a_base_declaring_255_extensions_is_read_to_four_blocks_and_no_further() {
    let t = Ticks::new();
    let mut e = honest(6);
    (e[126], e[127]) = (255, e[127].wrapping_sub(255 - 5));
    let mut s = Sink::new(&t, e.clone());
    let (n, out, _) = read(&mut s, &t);
    assert_eq!((n, &out[..]), (512, &e[..512]));
    assert_eq!(s.log.len(), 4, "one try a block");
}

#[test]
fn a_sink_holding_the_bus_costs_three_tries_inside_the_read_s_deadline() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(1));
    s.faults = vec![Fault::Stall(0); 12];
    let (n, _, took) = read(&mut s, &t);
    assert_eq!((n, s.log.len()), (0, 3));
    for x in &s.log {
        assert!(x.t1 - x.t0 <= XFER_US + 1_000, "one try within its deadline");
    }
    assert!((3 * XFER_US..EDID_US).contains(&took), "took {took}");
}

#[test]
fn a_sink_holding_the_clock_on_every_word_is_cut_at_each_try_s_deadline() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(1));
    s.faults = vec![Fault::Slow(4_000); 12];
    assert_eq!(read(&mut s, &t).0, 0);
    for x in &s.log {
        assert!(x.got.len() <= 4 * (XFER_US / 4_000) as usize, "no word after the deadline");
        assert!(x.t1 - x.t0 <= XFER_US + 5_000);
    }
}

#[test]
fn words_offered_past_the_block_are_never_taken() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(2));
    s.faults = vec![Fault::Over; 2];
    assert_eq!(read(&mut s, &t).0, 256);
    assert!(s.log.iter().all(|x| x.got.len() == 128), "32 words a block, no more");
}
