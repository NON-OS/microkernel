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
//! The microphone driver: a client of every USB host, started after the hub
//! walker, on the ports it was given, asking for no device of its own. Its
//! endpoint is not held: it is the microphone gate itself and asks the kernel
//! about MicrophoneCapture at every open and start.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};
use super::usb_hosts::USB_HOSTS;

const SPAWN: &str = include_str!("../../../../src/hardware/usb_mic_capsule/spawn.rs");

#[test]
fn the_microphone_driver_starts_after_the_hub_walker_where_a_host_is() {
    let usb = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb.rs");
    let at = |s: &str| usb.find(s).expect(s);
    assert!(at("drivers_usb_hub::spawn();") < at("drivers_usb_mic::spawn();"));
    let plan = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb_mic.rs");
    assert!(plan.contains("if !super::usb_host::usb_host() {"));
    for line in [
        "const SERVICE_NAME: &str = \"driver.usb_mic0\";",
        "const SERVICE_PORT: u32 = 4520;",
        "const REPLY_PORT: u32 = 4521;",
        "const REPLY_INBOX: &str = \"endpoint.4294967481\";",
    ] {
        assert!(SPAWN.contains(line), "{line}");
    }
}

#[test]
fn the_microphone_driver_asks_for_no_device_and_no_microphone_bit() {
    for absent in ["Mmio", "Dma", "Irq", "Pio", "Driver", "DeviceEnum", "Microphone"] {
        assert!(!SPAWN.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}

#[test]
fn the_microphone_driver_reaches_every_host_and_its_gate_is_its_own() {
    for host in USB_HOSTS {
        assert!(endpoint_admits(host, owner(&["driver.usb_mic0"])), "{host}");
    }
    assert_eq!(callers_of("driver.usb_mic0"), None, "the bit is the gate");
}
