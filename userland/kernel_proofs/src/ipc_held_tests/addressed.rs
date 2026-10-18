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

//! K7: a send addressed by any name of another capsule's lands in its one
//! inbox, so it takes every gate that inbox serves. On the old kernel the
//! reply endpoint alone was checked, needing only IPC: each refusal below
//! was a delivery there.

use super::common::{owner, IPC, NETWORK};
use super::held::inbox_admits;
use super::registry_addressed::addressed_admits;

/// A capsule addressing `name`, served by a process with `served`, the
/// caller holding `held` and owning `mine`.
fn send(served: &[(&str, u64)], held: u64, mine: &'static [&'static str], name: &str) -> bool {
    let own = owner(mine)(name);
    addressed_admits(false, own, || inbox_admits(served.iter().copied(), held, owner(mine)))
}

const XHCI: [(&str, u64); 2] = [("driver.xhci0", IPC), ("endpoint.driver.xhci0.reply", IPC)];
const NET: [(&str, u64); 2] = [("net.sockets", IPC | NETWORK), ("endpoint.net.sockets.reply", IPC)];

#[test]
fn a_held_driver_is_not_reached_by_its_reply_endpoint() {
    let reply = "endpoint.driver.xhci0.reply";
    assert!(!send(&XHCI, IPC, &[], reply), "IPC alone reached xhci's bulk-IN");
    assert!(!send(&XHCI, IPC | NETWORK, &["app.attack"], reply), "nor with more caps");
    assert!(send(&XHCI, IPC, &["driver.usb_hid0"], reply), "its own client still does");
    assert!(send(&XHCI, IPC, &["driver.usb_msc0"], "driver.xhci0"));
}

#[test]
fn a_net_service_is_not_reached_by_its_reply_endpoint_without_network() {
    let reply = "endpoint.net.sockets.reply";
    assert!(!send(&NET, IPC, &[], reply), "IPC alone reached net.sockets");
    assert!(send(&NET, IPC | NETWORK, &[], reply), "Network passes, as by pid");
}

#[test]
fn the_kernels_endpoints_and_the_callers_own_are_unchanged() {
    assert!(addressed_admits(true, false, || false), "a kernel-owned reply inbox");
    let mine: &'static [&'static str] = &["app.attack", "endpoint.app.attack.reply"];
    let own = [("app.attack", IPC), ("endpoint.app.attack.reply", IPC)];
    assert!(send(&own, IPC, mine, "endpoint.app.attack.reply"), "its own reply port");
    let mut asked = false;
    assert!(addressed_admits(false, true, || {
        asked = true;
        false
    }));
    assert!(!asked, "the owner's gates are read only for another capsule");
}
