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

//! The two orders the Bus Master tests hold the driver to: Bus Master on
//! only after a reset, with the engines off and the rings this driver's;
//! and nothing given back before the part is stopped, reset and off the bus.

use nonos_libc::{command, Seen, Step, MK_PCI_CMD_BUS_MASTER as BM};

/// One bring-up's log, with `resets` done by its end: none while Bus Master
/// was on, and it went on once, after a reset, with both engines off and both
/// rings naming this driver's frames.
pub fn bus_master_after_reset_and_rings(seen: &[Seen], resets: u32) {
    // Each step carries Bus Master as it stood since the step before it; the
    // stretch after the last step runs to now.
    let stretches = seen.iter().map(|s| (s.bus_master, s.part.resets));
    let mut done = 0;
    for (on, by) in stretches.chain([(command() & BM != 0, resets)]) {
        assert!(!on || by == done, "the part was reset with Bus Master on: {seen:?}");
        done = by;
    }
    let on = |s: &&Seen| matches!(s.step, Step::Command(v) if v & BM != 0);
    let on: Vec<&Seen> = seen.iter().filter(on).collect();
    assert_eq!(on.len(), 1, "Bus Master turned on once by the bring-up: {seen:?}");
    let p = on[0].part;
    assert!(p.resets > 0, "Bus Master went on before the reset: {p:?}");
    assert!(!p.rx_on && !p.tx_on, "Bus Master went on with an engine running: {p:?}");
    assert!(p.rings_own, "Bus Master went on before the rings were named: {p:?}");
}

/// A teardown's log: the device itself is the first thing given back, with
/// Bus Master already off, both engines stopped and at least `resets` resets
/// done, so no frame is freed while the part could still reach it.
pub fn released_quiet(seen: &[Seen], resets: u32) {
    let first = seen.iter().find(|s| matches!(s.step, Step::Freed(..)));
    let first = first.expect("the attempt gave the device back");
    let device_first = matches!(first.step, Step::Freed("device_release", _));
    assert!(device_first, "a grant went back before the device: {seen:?}");
    assert!(!first.bus_master, "released with Bus Master on: {seen:?}");
    assert!(!first.part.rx_on && !first.part.tx_on, "released with an engine on: {seen:?}");
    assert!(first.part.resets >= resets, "released without the reset: {seen:?}");
}
