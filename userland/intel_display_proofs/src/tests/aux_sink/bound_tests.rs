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

//! The bounds over AUX: a DEFER storm and a stalled sink end within their
//! tries and the read's deadline; a reply longer than asked never lands
//! past the block; a stop never acknowledged loses the block.

use super::edge_tests::read;
use super::sink::{AuxSink, Fault, Rates};
use crate::dp::native::RETRIES;
use crate::engine::gmbus_xfer::deadline::XFER_US;
use crate::tests::ddc_sink::kit::honest;

#[test]
fn a_defer_storm_ends_within_its_retries() {
    let mut s = AuxSink::new(honest(4));
    s.rates = Rates { defer: 100, ..Rates::default() };
    let (n, _, s) = read(s);
    assert_eq!(n, 0);
    // Three tries at the base: its offset write and stop, each RETRIES times.
    assert_eq!(s.transfers, 3 * 2 * RETRIES, "{}", s.transfers);
}

#[test]
fn a_stalled_dp_sink_costs_three_tries_each_within_its_deadline() {
    let mut s = AuxSink::new(honest(1));
    s.faults = vec![Fault::Stall(0); 12];
    let (n, _, s) = read(s);
    assert_eq!((n, s.log.len()), (0, 3));
    let stop = (RETRIES as u64 + 1) * 3_000;
    assert!(s.log.iter().all(|t| t.t1 - t.t0 <= XFER_US + stop), "each try ends");
}

#[test]
fn long_replies_never_land_past_the_block_and_an_unacknowledged_stop_loses_it() {
    let mut s = AuxSink::new(honest(2));
    s.rates = Rates { odd: 2, ..Rates::default() };
    let (n, out, s) = read(s);
    assert_eq!((n, &out[..]), (256, &[&honest(2)[..], &[0xA5; 256][..]].concat()[..]));
    assert!(s.log.iter().all(|t| t.got.len() <= 128));
    let mut s = AuxSink::new(honest(1));
    s.faults = vec![Fault::NoPhase; 3];
    assert_eq!(read(s).0, 0, "a stop NACKed three times running");
}
