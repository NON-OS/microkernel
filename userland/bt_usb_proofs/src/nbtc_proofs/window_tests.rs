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

//! PAIR and SCAN: a window one new device may pair in, shut when it has,
//! when it times out, or when scanning stops.

use super::fixture::{ask, board, KB};
use crate::nbtc::ops::*;
use crate::nbtc::who::Caller::Settings;

#[test]
fn the_pairing_window_starts_shut_and_opens_for_pair_ms() {
    let mut b = board();
    assert!(!b.may_pair(0));
    assert_eq!(ask(&mut b, Settings, OP_PAIR, &[], 1_000), (E_OK, vec![]));
    assert!(b.may_pair(1_000) && b.may_pair(1_000 + PAIR_MS - 1));
    assert!(!b.may_pair(1_000 + PAIR_MS), "it closes on its own");
    assert_eq!(ask(&mut b, Settings, OP_LINK, &[], 1_000).1[1] & 0b1_0000, 0b1_0000);
}

#[test]
fn one_device_paired_shuts_the_window_and_a_resumed_bond_does_not() {
    let mut b = board();
    ask(&mut b, Settings, OP_PAIR, &[], 0);
    b.linked(KB, false);
    assert!(b.may_pair(10), "re-encrypting a kept bond lets nobody in");
    b.linked(KB, true);
    assert!(!b.may_pair(10));
}

#[test]
fn pair_turns_scanning_on_and_stopping_it_shuts_the_window() {
    let mut b = board();
    assert_eq!(ask(&mut b, Settings, OP_SCAN, &[0], 0), (E_OK, vec![]));
    assert!(!b.scan);
    ask(&mut b, Settings, OP_PAIR, &[], 0);
    assert!(b.scan && b.may_pair(1));
    ask(&mut b, Settings, OP_SCAN, &[0], 0);
    assert!(!b.may_pair(1));
    assert_eq!(ask(&mut b, Settings, OP_SCAN, &[1], 0).0, E_OK);
    assert!(b.scan);
}

#[test]
fn scan_takes_one_octet_of_zero_or_one() {
    let mut b = board();
    for wrong in [&[][..], &[2], &[1, 1]] {
        assert_eq!(ask(&mut b, Settings, OP_SCAN, wrong, 0).0, E_INVAL, "{wrong:?}");
    }
    assert!(b.scan);
}
