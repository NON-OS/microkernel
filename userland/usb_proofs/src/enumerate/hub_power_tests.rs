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

//! A new hub's ports are looked at once their power is good, a deadline the
//! loop checks, not a sleep: the driver slept through it, up to 510 ms with
//! no report read, each time a hub was plugged in.

use super::devices::Devices;
use super::scan_hub::scan_hub;
use crate::hub::{Hub, Place};
use crate::xhci::sim;
use nonos_libc::{advance_ms, idled_ms, mk_uptime_ms, reset};

#[test]
fn a_new_hubs_ports_wait_for_power_good_on_the_clock() {
    reset();
    // Every port connected for as long as the test runs.
    sim::flapping(1_000_000);
    let (mut devs, mut hub) = (Devices::new(), Hub::new(1, Place::root(1, 3), 4));
    hub.ready_ms = 300;
    let mut first_reset = None;
    while mk_uptime_ms() < 1_000 {
        let busy = scan_hub(7, &mut hub, &mut devs, false);
        assert!(busy || mk_uptime_ms() >= 300, "not powered yet, so looked at again");
        if first_reset.is_none() && sim::resets()[1] > 0 {
            first_reset = Some(mk_uptime_ms());
        }
        advance_ms(1);
    }
    assert_eq!(idled_ms(), 0, "never asleep");
    assert_eq!(first_reset, Some(400), "power good at 300, then 100 ms of debounce");
}
