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

//! A device that stops completing transmits: told apart from a
//! busy one by elapsed time with the link up, and then named Stalled so the
//! serve loop resets it, never left as a ring stuck full. A completion that
//! arrives late, before the time is up, ends the count.

use super::drive::driver;
use super::harness::Dev;
use crate::clock::set;
use crate::constants::{TX_DESC_COUNT, TX_STALL_MS};
use crate::queue::TxError;
use crate::setup::Driver;
use crate::tx::send;

pub fn full(d: &mut Driver) {
    for _ in 0..TX_DESC_COUNT {
        assert_eq!(send(d, &[0; 60]), Ok(()));
    }
}

#[test]
fn a_ring_full_with_the_link_up_is_stalled_only_once_the_time_is_up() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    full(&mut d);
    set(1_000);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full));
    set(1_000 + TX_STALL_MS - 1);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full));
    set(1_000 + TX_STALL_MS);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Stalled));
}

#[test]
fn a_late_completion_before_the_time_is_up_ends_the_count() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    full(&mut d);
    set(0);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full));
    tx.complete(0, 3, 0);
    set(TX_STALL_MS - 1);
    assert_eq!(send(&mut d, &[0; 60]), Ok(()), "the late completion freed a buffer");
    set(TX_STALL_MS + 5);
    assert_eq!(send(&mut d, &[0; 60]), Err(TxError::Full), "the count started again");
}
