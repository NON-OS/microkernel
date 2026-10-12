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
//! Every USB host is held to the same class drivers, and every host the
//! kernel spawns is on the list the class drivers look up.

use super::common::owner;
use super::held::{callers_of, endpoint_admits};
use super::usb_hosts::USB_HOSTS;

/// Each spawn's file and the name it registers.
const SPAWNS: &[(&str, &str)] =
    &[(include_str!("../../../../src/hardware/xhci_capsule/spawn.rs"), "driver.xhci0")];

const APPS: &[&str] = &["app.browser", "app.terminal", "compositor", "net.core"];

#[test]
fn every_spawned_host_is_listed_and_held() {
    for (spawn, name) in SPAWNS {
        let line = format!("const SERVICE_NAME: &str = \"{name}\";");
        assert!(spawn.contains(&line), "{name} is not what its spawn registers");
        assert!(USB_HOSTS.contains(name), "{name} is not on the host list");
        assert!(callers_of(name).is_some(), "{name} is not held");
    }
}

#[test]
fn every_listed_host_takes_the_class_drivers_and_no_one_else() {
    let first = callers_of(USB_HOSTS[0]).expect("held");
    for want in ["driver.usb_hid0", "driver.usb_msc0", "driver.usb_audio0", "driver.rtl8153_0"] {
        assert!(first.contains(&want), "{want} cannot reach the host");
    }
    for host in USB_HOSTS {
        assert_eq!(callers_of(host), Some(first), "{host} is held to another list");
        assert!(!endpoint_admits(host, owner(APPS)), "{host} took an app");
    }
}

#[test]
fn the_hosts_are_fastest_first_and_each_named_once() {
    assert_eq!(USB_HOSTS, ["driver.xhci0", "driver.ehci0", "driver.ohci0", "driver.uhci0"]);
}
