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

//! `power.off` has one owner, capsule_power (P-08): capsule_acpi neither
//! declares the name, nor has the kernel register it for it, nor serves a
//! POFF frame. Its own power-offs (button, _CRT, battery) stay its own.

const MK: &str = include_str!("../../capsule_acpi/Capsule.mk");
const REPLY: &str = include_str!("../../capsule_acpi/src/server/reply.rs");
const OFF: &str = include_str!("../../capsule_acpi/src/power/off/mod.rs");
const MIRROR: &str = include_str!("../../../src/userspace/capsule_acpi/mod.rs");
const SPAWN: &str = include_str!("../../../src/userspace/capsule_acpi/spawn.rs");
const POWER_MK: &str = include_str!("../../capsule_power/Capsule.mk");

#[test]
fn capsule_acpi_holds_no_power_off_name() {
    assert!(!MK.contains("power.off"), "Capsule.mk declares no power.off");
    assert!(!MK.contains("CAPSULE_INSTANCE_ENDPOINTS"), "nor any second name");
    assert!(!MIRROR.contains("power_off") && !SPAWN.contains("power_off"), "the kernel registers none");
    assert!(POWER_MK.contains("service:4446:power.off"), "capsule_power still owns it");
}

#[test]
fn capsule_acpi_serves_no_poff_frame_and_keeps_its_own_power_offs() {
    assert!(!REPLY.contains("is_poff") && !REPLY.contains("requested("), "no POFF branch");
    assert!(!OFF.contains("mod request") && !OFF.contains("mod wire"), "no request server");
    assert!(OFF.contains("pub use serve::{power_off, serve, Why};"), "button, _CRT, battery");
}
