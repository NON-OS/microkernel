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

//! The stall model run long: a backend that holds every transmit buffer
//! with the link up, net.core sending every 100 ms, and after each reset
//! the backoff allows, a fresh driver over cleared rings, as bring-up makes.

use super::drive::driver;
use super::harness::Dev;
use super::tx_stall_tests::full;
use crate::clock::set;
use crate::constants::VQ_REGION_SIZE;
use crate::queue::TxError;
use crate::server::backoff::{Backoff, Step};
use crate::tx::send;

/// Each reset the backoff allows in `until` ms, and whether it was logged.
/// After reset number `revive`, the device completes one frame.
pub fn stalls(until: u64, revive: usize) -> Vec<(u64, bool)> {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let (mut d, mut backoff, mut resets) = (driver(&rx, &tx), Backoff::default(), Vec::new());
    full(&mut d);
    for now in (0..until).step_by(100) {
        set(now);
        if send(&mut d, &[0; 60]) != Err(TxError::Stalled) {
            continue;
        }
        if let Step::Reset { log } = backoff.step(now, d.tx.completed) {
            resets.push((now, log));
            tx.ring.put(0, &[0; VQ_REGION_SIZE]);
            d = driver(&rx, &tx);
            full(&mut d);
            if resets.len() == revive {
                tx.complete(0, 0, 0);
            }
        }
    }
    resets
}
