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
//! Master: each ends by name, with no queue left running on the part.

use nonos_libc::{entropy, refuse_bus_master, said, seen};

use super::bus_master_tests::claimed;
use super::memory::Memory;
use super::order::released_quiet;
use super::tx_hang_tests::stalled;
use crate::constants::ctrl::STATUS_LU;
use crate::init::finish;
use crate::server::tx_watch::check;

#[test]
fn a_reset_that_fails_is_named_and_tried_again_later() {
    let _turn = entropy(true);
    let (part, _resets, mut d, _mem) = stalled(STATUS_LU);
    drop(part);
    assert!(check(&mut d, false), "the hang is acted on");
    let line = "igc: tx hang, reset: CTRL.RST did not self-clear in 100 ms\n";
    assert_eq!(said().last().unwrap(), line);
    assert!(!d.tx_watch.expired(), "the next try waits a whole bound");
}

#[test]
fn a_refused_bus_master_ends_the_attempt_with_the_queues_never_on() {
    let _turn = entropy(true);
    let (bar, _part, _resets) = claimed(|_| {});
    refuse_bus_master();
    let err = finish(Memory::new().driver(&bar)).err();
    assert_eq!(err, Some("PCI command write (bus master) refused"));
    assert!(seen().iter().all(|s| !s.part.rx_on && !s.part.tx_on), "{:?}", seen());
    released_quiet(&seen(), 2);
}
