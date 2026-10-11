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

//! The compositor finds this driver: its graphics list names the service the
//! capsule registers, after the virtio GPU's.

fn handle(mk: &str) -> &str {
    let line = mk.lines().find_map(|l| l.strip_prefix("CAPSULE_HANDLE")).expect("handle");
    line.trim().trim_start_matches(":=").trim()
}

#[test]
fn the_compositors_graphics_list_names_both_gfx_drivers_gpu_first() {
    let discover = include_str!("../../compositor/src/setup/discover.rs");
    let list = discover
        .lines()
        .find(|l| l.contains("GFX_SERVICES: [&[u8]; 2] ="))
        .expect("the compositor's graphics list");
    let bga = handle(include_str!("../../capsule_driver_bga/Capsule.mk"));
    let gpu = handle(include_str!("../../capsule_driver_virtio_gpu/Capsule.mk"));
    let at = |name: &str| list.find(&format!("b\"{name}\"")).unwrap_or_else(|| panic!("{name}"));
    assert!(at(gpu) < at(bga), "{list}");
}
