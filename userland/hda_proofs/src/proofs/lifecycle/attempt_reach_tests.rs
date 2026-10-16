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

//! The attempts that end on memory a 32-bit controller cannot reach, before
//! the rings run and after, and a driver that played and is then dropped
//! with its stream still running: each lets go in the same order.

use nonos_libc::calls;

use crate::constants::{DPLBASE, DPLBASE_ENABLE, GCAP, SDCTL_RUN};
use crate::error::HdaError;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::lifecycle::attempt::{attempt, attempt_until};
use crate::proofs::lifecycle::board::{board, Board, LOW};
use crate::proofs::lifecycle::given_back::quiesced_then_released;
use crate::proofs::lifecycle::teardown::sd_ctl;
use crate::protocol::{Request, OP_STREAM_START};
use crate::server::handlers::stream::handle_stream_start;
use crate::setup::Started;

const HIGH: u64 = 1 << 32;

/// A controller without 64-bit addressing, given buffers at `dma`.
fn narrow(dma: &[u64]) -> Board {
    let b = board(vec![(0, sim_codec(&alc236_hp()))], dma);
    b.sim.bar.present16(GCAP as usize, 0x4400);
    b
}

#[test]
fn rings_above_4_gib_are_refused_and_the_controller_let_go_in_order() {
    let (r, log) = attempt(&narrow(&[HIGH, HIGH + 0x10_0000]));
    assert_eq!(r.err(), Some(HdaError::DmaOutOfReach));
    quiesced_then_released(&log);
}

#[test]
fn a_stream_above_4_gib_is_refused_with_the_rings_stopped_first() {
    let stream_high = || narrow(&[LOW[0], LOW[1], HIGH, HIGH + 0x10_0000]);
    let (_b, _, log) = attempt_until(stream_high, |r, _| matches!(r, Err(HdaError::DmaOutOfReach)));
    quiesced_then_released(&log);
}

#[test]
fn a_driver_dropped_while_its_stream_plays_stops_it_before_the_release() {
    let alc236 = || board(vec![(0, sim_codec(&alc236_hp()))], &LOW);
    let (b, r, _) = attempt_until(alc236, |r, _| matches!(r, Ok(Started::Playing(_))));
    let Ok(Started::Playing(mut d)) = r else { panic!("the ALC236 did not play") };
    let req = Request { op: OP_STREAM_START, flags: 0, request_id: 1, payload_len: 0 };
    let mut running = false;
    handle_stream_start(&mut d, &req, &mut [0u8; 64], &mut running);
    assert!(running, "the stream did not start");
    assert_ne!(b.sim.bar.wrote8(sd_ctl(4)) & SDCTL_RUN, 0, "the output engine is not running");
    assert_ne!(b.sim.bar.wrote32(DPLBASE as usize) & DPLBASE_ENABLE, 0, "no position buffer");
    let before = calls().len();
    drop(d);
    quiesced_then_released(&calls()[before..]);
}
