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

//! An EDID read cut, repeated or shifted mid-block, each with what the
//! driver keeps (segment pointer lies in `eddc_tests.rs`).

use super::kit::{honest, read};
use super::sink::{Fault, Sink};
use crate::model::Ticks;

#[test]
fn a_block_cut_mid_read_is_read_again_and_kept_whole() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(1));
    s.faults = vec![Fault::Nak(16), Fault::Stall(9)];
    let (n, out, _) = read(&mut s, &t);
    assert_eq!((n, &out[..128]), (128, &honest(1)[..]));
    assert_eq!(s.log.len(), 3, "two cut tries, then the whole block");
}

#[test]
fn a_shifted_block_never_lands_and_three_shifts_end_the_read() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(2));
    s.faults = vec![Fault::None, Fault::Drop(40), Fault::Again(3), Fault::Index(1)];
    let (n, out, _) = read(&mut s, &t);
    assert_eq!(n, 128, "block 1 shifted three times running is not kept");
    assert!(out[128..].iter().all(|&b| b == 0xA5), "no try's bytes reach the blob");
}

#[test]
fn a_sink_ignoring_its_index_repeats_the_base_and_that_is_refused() {
    let t = Ticks::new();
    let mut s = Sink::new(&t, honest(2));
    s.ignore_index = true;
    assert_eq!(read(&mut s, &t).0, 128);
    assert_eq!(s.log.len(), 4, "the base, then three tries at block 1");
}
