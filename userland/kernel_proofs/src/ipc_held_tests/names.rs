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

//! The names on the lists are the names the kernel registers.

use super::common::{WIFI, WIRED};
use super::held::callers_of;

/// Every card a boot profile counts as a network driver is held, under the
/// name its spawn registers.
#[test]
fn the_list_names_every_card_the_kernel_starts() {
    let refuse = include_str!(
        "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs"
    );
    let start = refuse.find("const NETWORK_DRIVERS").expect("the network driver list");
    let list = &refuse[start..start + refuse[start..].find("];").expect("its end")];
    let named: Vec<&str> = list.split('"').skip(1).step_by(2).collect();
    assert_eq!(named.len(), 6, "{named:?}");
    for card in &named {
        assert!(callers_of(card).is_some(), "{card} is a network driver and not held");
    }
    let spawns = [
        include_str!("../../../../src/hardware/virtio_net_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/e1000_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8169_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8139_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/iwlwifi_capsule/spawn.rs"),
        include_str!("../../../../src/hardware/rtl8821ce_capsule/spawn.rs"),
    ];
    for (card, spawn) in WIRED.iter().chain(WIFI.iter()).zip(spawns) {
        let line = format!("const SERVICE_NAME: &str = \"{card}\";");
        assert!(spawn.contains(&line), "{card} is not the name its spawn registers");
    }
}

/// The stack's names are the endpoints the kernel registers for it.
#[test]
fn the_callers_are_the_names_the_kernel_registers() {
    let spawns = [
        ("net.core", include_str!("../../../../src/userspace/capsule_net_core/spawn.rs")),
        ("net.l2", include_str!("../../../../src/userspace/capsule_net_l2/spawn.rs")),
        ("app.settings", include_str!("../../../../src/userspace/capsule_settings/spawn.rs")),
        (
            "app.setup_wizard",
            include_str!("../../../../src/userspace/capsule_setup_wizard/spawn.rs"),
        ),
    ];
    for (name, spawn) in spawns {
        let line = format!("const SERVICE_NAME: &str = \"{name}\";");
        assert!(spawn.contains(&line), "{name} is not what its spawn registers");
    }
    let settings = spawns[2].1;
    for window in ["\"app.settings.1\"", "\"app.settings.2\""] {
        assert!(settings.contains(window), "Settings has no window {window}");
    }
    assert!(!settings.contains("\"app.settings.3\""), "Settings grew a window the list lacks");
}
