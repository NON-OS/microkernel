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

//! `power.off` (F-88): held to the desktop shell alone, registered by
//! capsule_power's spawn under the name and port its Capsule.mk declares,
//! and claimed by no other capsule. It shares that capsule's inbox with
//! `power`, so a send straight to that pid must pass both gates, which only
//! the shell does.

use super::held::{callers_of, endpoint_admits, inbox_admits};

const SHELL: &str = "desktop_shell";

#[test]
fn only_the_shell_asks_for_the_wiped_power_off() {
    assert_eq!(callers_of("power.off"), Some(&[SHELL][..]));
    for caller in ["power", "driver.i2c_pci0", "driver.gpio0", "app.settings", "acpi", ""] {
        assert!(!endpoint_admits("power.off", |n| n == caller), "{caller}");
    }
    assert!(endpoint_admits("power.off", |n| n == SHELL));
    let served = [("power", 0x8u64), ("power.off", 0x8)];
    assert!(inbox_admits(served, 0x8, |n| n == SHELL), "the shell by pid");
    assert!(!inbox_admits(served, 0x8, |n| n == "acpi"), "acpi by pid: power.off refuses");
}

#[test]
fn the_name_is_the_one_registered_and_declared() {
    let spawn = include_str!("../../../../src/userspace/capsule_power/power_off.rs");
    assert!(spawn.contains("const NAME: &str = \"power.off\";"));
    assert!(spawn.contains("const PORT: u32 = 4446;"));
    let mk = include_str!("../../../capsule_power/Capsule.mk");
    assert!(mk.contains("CAPSULE_INSTANCE_ENDPOINTS := service:4446:power.off"));
    let acpi = include_str!("../../../capsule_acpi/Capsule.mk");
    assert!(!acpi.contains("power.off"), "the ACPI capsule claims no second name");
    let shell = include_str!("../../../../src/userspace/capsule_desktop_shell/spawn.rs");
    assert!(shell.contains("\"desktop_shell\""), "the shell registers the name HELD names");
}
