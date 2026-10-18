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
//! Master: each ends by name, with nothing left running on the part.

use std::sync::atomic::{AtomicU32, Ordering::SeqCst};
use std::sync::Arc;

use nonos_libc::{entropy, logged, refuse_bus_master, seen, set_command, watch};

use super::memory::Memory;
use super::model::{live, window};
use super::order::released_quiet;
use super::probe::{counting, probe};
use super::tx_hang_tests::stalled;
use crate::init::finish;
use crate::server::tx_watch::check;

#[test]
fn a_reset_that_fails_is_named_and_tried_again_later() {
    let _turn = entropy(true);
    let resets = Arc::new(AtomicU32::new(0));
    let (_bar, part, mut d, _mem) = stalled(&resets);
    drop(part);
    assert!(check(&mut d, true), "the hang is acted on");
    let line = "rtl8169: tx hang, reset: rtl8169 reset timeout\n";
    assert_eq!(logged().last().unwrap(), line);
    assert!(!d.tx_watch.expired(), "the next try waits a whole bound");
}

#[test]
fn a_refused_bus_master_ends_the_attempt_with_the_engines_never_on() {
    let _turn = entropy(true);
    let (bar, resets) = (window(), Arc::new(AtomicU32::new(0)));
    let _part = live(&bar, counting(&resets));
    set_command(0);
    refuse_bus_master();
    watch(probe(&bar, &resets));
    let err = finish(Memory::new().driver(&bar)).err();
    assert_eq!(err, Some("pci command write refused (bus master)"));
    assert!(seen().iter().all(|s| !s.part.rx_on && !s.part.tx_on), "{:?}", seen());
    released_quiet(&seen(), 2);
    assert_eq!(resets.load(SeqCst), 2, "reset at bring-up and again at release");
}
