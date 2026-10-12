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
//! The USB security key driver: a client of every USB host, started after
//! the hub walker, on the ports it was given, asking for no device of its
//! own, and held to no one: a key signs for whoever reaches it once its
//! user touches it, so until the owner names who may ask, nothing does.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};
use super::usb_hosts::USB_HOSTS;

const SPAWN: &str = include_str!("../../../../src/hardware/fido_capsule/spawn.rs");

#[test]
fn the_key_driver_starts_after_the_hub_walker_where_a_host_is() {
    let usb = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb.rs");
    let at = |s: &str| usb.find(s).expect(s);
    assert!(at("drivers_usb_hub::spawn();") < at("drivers_fido::spawn();"));
    let plan = include_str!("../../../../src/userspace/init/spawn_plan/drivers_fido.rs");
    assert!(plan.contains("if !super::usb_host::usb_host() {"));
    for line in [
        "const SERVICE_NAME: &str = \"driver.fido0\";",
        "const SERVICE_PORT: u32 = 4526;",
        "const REPLY_PORT: u32 = 4527;",
        "const REPLY_INBOX: &str = \"endpoint.4294967484\";",
    ] {
        assert!(SPAWN.contains(line), "{line}");
    }
    for absent in ["Mmio", "Dma", "Irq", "Pio", "Driver", "DeviceEnum"] {
        assert!(!SPAWN.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}

#[test]
fn the_key_driver_reaches_every_host_and_nothing_reaches_it() {
    for host in USB_HOSTS {
        assert!(endpoint_admits(host, owner(&["driver.fido0"])), "{host}");
    }
    assert_eq!(callers_of("driver.fido0"), Some(&[][..]));
    let others = owner(&["app.browser", "app.terminal", "net.core", "compositor"]);
    assert!(!endpoint_admits("driver.fido0", others), "a capsule reaches it");
}
