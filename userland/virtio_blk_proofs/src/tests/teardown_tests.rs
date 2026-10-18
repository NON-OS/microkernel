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

//! Teardown: the device is reset and seen stopped, bus
//! mastering is switched off, and the claim is given back last; the driver
//! unmaps nothing itself, so no frame is freed while the device can reach
//! it. The shim has no MkDmaUnmap: a driver file that called it would not
//! build here.

use std::cell::Cell;
use std::rc::Rc;

use nonos_libc::{events, on_event, Event};

use super::bench::bench;
use super::bringup_tests::masters;
use crate::setup;

#[test]
fn teardown_resets_then_stops_bus_mastering_then_gives_the_device_back() {
    let dev = bench(8, 2048, 1);
    dev.borrow_mut().msix = true;
    let driver = setup::run().expect("the device comes up");
    let at_release = Rc::new(Cell::new(None));
    let (watch, seen) = (dev.clone(), at_release.clone());
    on_event(move |e| {
        if e == Event::Release {
            seen.set(Some((watch.borrow().queue_live(), watch.borrow().seen_stopped)));
        }
    });
    assert!(driver.quiesce(), "the device read back reset and was taken back");
    assert_eq!(at_release.get(), Some((false, true)), "given back with its queue running");
    let log = events();
    let tail = &log[log.len() - 2..];
    assert!(matches!(tail[0], Event::Command(_)) && !masters(tail[0]), "{log:?}");
    assert_eq!(tail[1], Event::Release);
}
