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

//! A hung transmitter whose reset fails, and a broker that refuses Bus
//! Master: each ends by name, with neither unit left running.

use std::sync::atomic::Ordering;

use nonos_libc::{entropy, logged, refuse_bus_master, seen};

use super::bus_master_tests::{claimed, up};
use super::order::released_quiet;
use super::tx_hang_tests::stalled;
use crate::constants::status::STATUS_LU;
use crate::model::memory::Memory;
use crate::server::tx_watch::check;

#[test]
fn a_reset_that_fails_is_named_and_tried_again_later() {
    let _turn = entropy(true);
    let (part, mut d, _mem) = stalled(STATUS_LU);
    part.how.reset_stuck.store(true, Ordering::SeqCst);
    assert!(check(&mut d, false), "the hang is acted on");
    let line = "e1000e: tx hang, reset: reset did not clear in 50 ms";
    assert_eq!(logged().last().unwrap(), line);
    assert!(!d.tx_watch.expired(), "the next try waits a whole bound");
}

#[test]
fn a_refused_bus_master_ends_the_attempt_with_the_units_never_on() {
    let _turn = entropy(true);
    let (bar, _part) = claimed();
    refuse_bus_master();
    let err = up(&bar, &mut Memory::new()).err();
    assert_eq!(err, Some("pci command write refused (bus master)"));
    assert!(seen().iter().all(|s| !s.part.rx_on && !s.part.tx_on), "{:?}", seen());
    released_quiet(&seen(), 2);
}
