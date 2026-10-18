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

//! The compositor finds this driver by the handle its Capsule.mk declares:
//! the compositor's list of display drivers names that handle, after the
//! one virtio-gpu's Capsule.mk declares.

fn handle(mk: &str) -> &str {
    let line = mk.lines().find_map(|l| l.strip_prefix("CAPSULE_HANDLE")).expect("a handle");
    line.trim().trim_start_matches(":=").trim()
}

#[test]
fn the_compositors_list_names_both_capsules_handles_virtio_gpu_first() {
    let order = include_str!("../../../compositor/src/setup/discover/order.rs");
    let list = order
        .lines()
        .find(|l| l.contains("GFX_SERVICES: [&[u8]; 2] ="))
        .expect("the compositor's list of display drivers");
    let bga = handle(include_str!("../../../capsule_driver_bga/Capsule.mk"));
    let gpu = handle(include_str!("../../../capsule_driver_virtio_gpu/Capsule.mk"));
    let at = |name: &str| list.find(&format!("b\"{name}\"")).unwrap_or_else(|| panic!("{name}"));
    assert!(at(gpu) < at(bga), "{list}");
}
