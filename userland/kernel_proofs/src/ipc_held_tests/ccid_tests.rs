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
//! The USB smart-card reader driver: a client of every USB host, started
//! after the hub walker, on the ports it was given, asking for no device of
//! its own, and held to no one: a card signs for whoever reaches its
//! reader, so until the owner names a client nothing reaches it.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};
use super::usb_hosts::USB_HOSTS;

const SPAWN: &str = include_str!("../../../../src/hardware/ccid_capsule/spawn.rs");

#[test]
fn the_reader_driver_starts_after_the_hub_walker_where_a_host_is() {
    let usb = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb.rs");
    let at = |s: &str| usb.find(s).expect(s);
    assert!(at("drivers_usb_hub::spawn();") < at("drivers_ccid::spawn();"));
    let plan = include_str!("../../../../src/userspace/init/spawn_plan/drivers_ccid.rs");
    assert!(plan.contains("if !super::usb_host::usb_host() {"));
    for line in [
        "const SERVICE_NAME: &str = \"driver.ccid0\";",
        "const SERVICE_PORT: u32 = 4524;",
        "const REPLY_PORT: u32 = 4525;",
        "const REPLY_INBOX: &str = \"endpoint.4294967483\";",
    ] {
        assert!(SPAWN.contains(line), "{line}");
    }
    for absent in ["Mmio", "Dma", "Irq", "Pio", "Driver", "DeviceEnum"] {
        assert!(!SPAWN.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}

#[test]
fn the_reader_driver_reaches_every_host_and_nothing_reaches_it() {
    for host in USB_HOSTS {
        assert!(endpoint_admits(host, owner(&["driver.ccid0"])), "{host}");
    }
    assert_eq!(callers_of("driver.ccid0"), Some(&[][..]));
    let others = owner(&["app.terminal", "app.browser", "net.core", "compositor"]);
    assert!(!endpoint_admits("driver.ccid0", others), "a capsule reaches it");
}
