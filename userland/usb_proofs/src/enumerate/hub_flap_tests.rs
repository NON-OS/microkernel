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

//! A hub whose ports flap, connected for a moment and then empty, for a
//! minute of the poll loop. Looking at its ports must never hold the loop in
//! a sleep, as no keyboard or mouse report is read meanwhile, and no port
//! may be tried more than its budget allows however often it reconnects.
//! The driver slept up to 230 ms in one look here, and reset a port 150
//! times in the minute.

use super::devices::Devices;
use super::scan_hub::scan_hub;
use crate::hub::budget::TRIES_PER_MINUTE;
use crate::hub::{Hub, Place};
use crate::poll::rescan::Rescan;
use crate::xhci::sim;
use nonos_libc::{advance_ms, idled_ms, mk_uptime_ms, reset};

const MINUTE_MS: i64 = 60_000;

#[test]
fn a_flapping_hub_never_holds_the_poll_loop_and_each_port_is_tried_a_few_times() {
    reset();
    sim::flapping(200);
    let mut devs = Devices::new();
    let mut hub = Hub::new(1, Place::root(1, 3), 15);
    let (mut longest, mut busy, mut ports) = (0u64, false, Rescan::new(0));
    // The poll loop's look at the ports, as `Rescan` paces it, on a pass
    // every millisecond: every port while quiet, between those the ports
    // mid bring-up.
    while mk_uptime_ms() < MINUTE_MS {
        let (before, now) = (idled_ms(), mk_uptime_ms() as u64);
        if ports.rescan(now) {
            busy = scan_hub(7, &mut hub, &mut devs, false);
        } else if busy && ports.hub_look(now) {
            busy = scan_hub(7, &mut hub, &mut devs, true);
        }
        longest = longest.max(idled_ms() - before);
        advance_ms(1);
    }
    let resets = sim::resets();
    let most = resets.iter().max().copied().unwrap_or(0);
    let said = format!("one look slept up to {longest} ms; a port was reset {most} times");
    assert!(longest == 0 && most <= u32::from(TRIES_PER_MINUTE), "{said}: {resets:?}");
    assert!(resets[1..=15].iter().all(|&r| r > 0), "every port was tried: {resets:?}");
}
