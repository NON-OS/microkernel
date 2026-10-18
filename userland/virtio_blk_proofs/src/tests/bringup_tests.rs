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

//! The legacy bring-up as it ships, against the device on the bench.
//! Bus Master goes on only once the queue a dead predecessor left running
//! is stopped and seen stopped (the device reset, its status read back as
//! zero), and never on a path that refuses the device.

use std::cell::Cell;
use std::rc::Rc;

use nonos_libc::{events, on_event, Event, MK_PCI_CMD_BUS_MASTER};

use super::bench::{bench, predecessor};
use crate::setup;

pub fn masters(e: Event) -> bool {
    matches!(e, Event::Command(bits) if bits & MK_PCI_CMD_BUS_MASTER != 0)
}

#[test]
fn bus_master_goes_on_only_once_a_predecessors_queue_is_stopped_and_seen_stopped() {
    let dev = bench(8, 2048, 1);
    dev.borrow_mut().msix = true;
    predecessor(&dev, 3);
    let at_master = Rc::new(Cell::new(None));
    let (watch, seen) = (dev.clone(), at_master.clone());
    // The first Bus Master write is the one that counts: a stray one
    // before the reset is not excused by a proper one after it.
    on_event(move |e| {
        if masters(e) && seen.get().is_none() {
            let d = watch.borrow();
            seen.set(Some((d.queue_live(), d.seen_stopped)));
        }
    });
    let driver = setup::run().expect("the device comes up");
    let why = "Bus Master went on over a queue still live, or one never seen stopped";
    assert_eq!(at_master.get(), Some((false, true)), "{why}");
    let d = dev.borrow();
    assert!(!d.written_mid_reset, "the handshake wrote over a reset still under way");
    assert!(d.queue_live() && d.pfn != 0x1234, "the device runs the old ring");
    assert_eq!(driver.capacity_sectors, 2048);
}

#[test]
fn a_device_that_never_finishes_its_reset_never_masters_the_bus() {
    let dev = bench(8, 2048, 1);
    dev.borrow_mut().msix = true;
    predecessor(&dev, u32::MAX);
    assert!(setup::run().is_err(), "a device that never stopped was brought up");
    let log = events();
    assert!(!log.iter().any(|&e| masters(e)), "Bus Master went on: {log:?}");
    assert_eq!(log.last(), Some(&Event::Release), "the claim was not given back last");
    assert!(!dev.borrow().written_mid_reset);
}
