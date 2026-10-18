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

//! When a full ring counts as a stopped device: only with the link up, and
//! then the serve loop is told to reset it rather than answer Full forever.

use super::drive::{call, driver, status};
use super::harness::Dev;
use super::tx_stall_tests::full;
use crate::clock::set;
use crate::constants::TX_STALL_MS;
use crate::protocol::{E_AGAIN, E_IO, OP_TX_PACKET};
use crate::queue::TxError;
use crate::regs::with;
use crate::tx::send;

#[test]
fn a_link_that_is_down_holds_frames_without_a_reset() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    full(&mut d);
    with(|m| m.link = 0);
    for now in [0, TX_STALL_MS, 10 * TX_STALL_MS] {
        set(now);
        assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full));
    }
    with(|m| m.link = 1);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full), "the count starts at link up");
}

#[test]
fn the_serve_loop_is_told_to_reset_a_stalled_device() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    full(&mut d);
    set(0);
    let (stalled, reply) = call(&mut d, OP_TX_PACKET, &[0; 60]);
    assert_eq!((stalled, status(&reply.unwrap())), (false, E_AGAIN));
    set(TX_STALL_MS);
    let (stalled, reply) = call(&mut d, OP_TX_PACKET, &[0; 60]);
    assert_eq!((stalled, status(&reply.unwrap())), (true, E_IO));
}
