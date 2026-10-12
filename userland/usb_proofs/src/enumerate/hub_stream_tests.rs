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

//! A hub that becomes ready while another device streams reports: the poll
//! loop then never rescans, and looks between reads only at ports already
//! mid bring-up. The new hub's ports were none of those, so a keyboard
//! behind it waited for the mouse to stop. Its first look once powered
//! takes every port.

use super::devices::Devices;
use super::scan_hub::scan_hub;
use crate::hub::{Hub, Place};
use crate::poll::rescan::Rescan;
use crate::xhci::sim;
use nonos_libc::{advance_ms, mk_uptime_ms, reset};

#[test]
fn a_new_hubs_ports_are_brought_up_while_a_mouse_streams() {
    reset();
    sim::flapping(1_000_000);
    let (mut devs, mut hub) = (Devices::new(), Hub::new(1, Place::root(1, 3), 4));
    hub.ready_ms = 300;
    // The rescan that attached the hub has just looked at it.
    let mut ports = Rescan::new(0);
    let mut busy = scan_hub(7, &mut hub, &mut devs, false);
    let mut first_reset = None;
    while mk_uptime_ms() < 3_000 && first_reset.is_none() {
        let now = mk_uptime_ms() as u64;
        // A report every millisecond: no rescan comes.
        ports.heard(now);
        if ports.rescan(now) {
            busy = scan_hub(7, &mut hub, &mut devs, false);
        } else if busy && ports.hub_look(now) {
            busy = scan_hub(7, &mut hub, &mut devs, true);
        }
        first_reset = (sim::resets()[1] > 0).then(mk_uptime_ms);
        advance_ms(1);
    }
    assert_eq!(first_reset, Some(400), "looked at once powered, at 300; reset after 100 ms");
}
