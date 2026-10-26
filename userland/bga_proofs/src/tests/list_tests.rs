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

//! bga in the kernel's display list: the compositor finds it there, by the
//! handle its Capsule.mk declares and after virtio-gpu's, and the kernel
//! holds it to the compositor because it is there.

use crate::displays::DISPLAYS;

#[test]
fn the_service_bga_serves_is_a_display() {
    assert!(DISPLAYS.contains(&"driver.bga0"));
}

#[test]
fn bga_comes_after_the_paravirtual_displays() {
    /*
     * virtio-gpu and the hypervisors' own displays carry hints and a
     * cursor; DISPI has neither, so it is the emulated fallback.
     */
    let at = |n| DISPLAYS.iter().position(|d| *d == n);
    assert!(at("driver.virtio_gpu0") < at("driver.bga0"));
}

/// The handle a Capsule.mk declares.
fn handle(mk: &str) -> &str {
    let line = mk.lines().find_map(|l| l.strip_prefix("CAPSULE_HANDLE")).expect("a handle");
    line.trim().trim_start_matches(":=").trim()
}

#[test]
fn the_list_names_both_capsules_handles_virtio_gpu_first() {
    let bga = handle(include_str!("../../../capsule_driver_bga/Capsule.mk"));
    let gpu = handle(include_str!("../../../capsule_driver_virtio_gpu/Capsule.mk"));
    let at = |n: &str| DISPLAYS.iter().position(|d| *d == n).unwrap_or_else(|| panic!("{n}"));
    assert!(at(gpu) < at(bga), "{DISPLAYS:?}");
}
