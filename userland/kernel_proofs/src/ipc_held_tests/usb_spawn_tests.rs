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

//! The spawn plan starts each USB host only where its controller is.

/// Each host's spawn asks the inventory for its own controller: OHCI's once
/// asked for a UHCI, so an OHCI-only machine never started it.
#[test]
fn each_host_spawns_where_its_own_controller_is() {
    let plan = [
        include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb_hosts.rs"),
        include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb_companions.rs"),
    ]
    .concat();
    for (host, family) in [("ehci", "UsbEhci"), ("ohci", "UsbOhci"), ("uhci", "UsbUhci")] {
        let start = plan.find(&format!("fn spawn_{host}() {{\n")).expect("its spawn");
        let body = &plan[start..start + plan[start..].find("\n}\n").unwrap()];
        assert!(body.contains(&format!("HardwareFamily::{family})")), "{host} asks for {family}");
        let others = ["UsbEhci", "UsbOhci", "UsbUhci"].into_iter().filter(|f| *f != family);
        for other in others {
            assert!(!body.contains(other), "{host}'s spawn asks for {other}");
        }
    }
}

/// The hub walker starts after the hosts (which the orchestrator starts
/// before the Wi-Fi, H-03) and before the class drivers, wherever a USB host
/// is, and registers the name the hosts are held to.
#[test]
fn the_hub_walker_starts_between_the_hosts_and_the_class_drivers() {
    let plan = include_str!("../../../../src/userspace/init/spawn_plan/orchestrator.rs");
    let usb = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb.rs");
    let at = |s: &str| usb.find(s).expect(s);
    let order = |s: &str| plan.find(s).expect(s);
    assert!(order("drivers_usb_hosts::spawn();") < order("super::drivers_usb::spawn();"));
    assert!(at("drivers_usb_hub::spawn();") < at("spawn_usb_hid();"));
    let hub = include_str!("../../../../src/userspace/init/spawn_plan/drivers_usb_hub.rs");
    assert!(hub.contains("if !super::usb_host::usb_host() {"));
    let spawn = include_str!("../../../../src/hardware/usb_hub_capsule/spawn.rs");
    assert!(spawn.contains("const SERVICE_NAME: &str = \"driver.usb_hub0\";"));
    for absent in ["Mmio", "Dma", "Irq", "Pio", "Driver", "DeviceEnum"] {
        assert!(!spawn.contains(&format!("Capability::{absent}")), "it asks for {absent}");
    }
}
