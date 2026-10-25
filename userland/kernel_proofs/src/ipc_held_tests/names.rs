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
