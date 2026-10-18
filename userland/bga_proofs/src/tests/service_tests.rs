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

//! The name and port bga announces: the ones Capsule.mk declares for its
//! spawn, and a name on the kernel's display list, which the compositor
//! walks to find it.

use nonos_libc::services;

use crate::announce::{announce, SERVICE_NAME, SERVICE_PORT};
use crate::displays::DISPLAYS;

const CAPSULE_MK: &str = include_str!("../../../capsule_driver_bga/Capsule.mk");

#[test]
fn the_service_announced_is_the_one_capsule_mk_declares() {
    let name = core::str::from_utf8(SERVICE_NAME).expect("a name in ASCII");
    let endpoint = format!("service:{SERVICE_PORT}:{name}");
    let declared = CAPSULE_MK.lines().any(|l| l.ends_with(&endpoint) && l.contains("SERVICE"));
    assert!(declared, "{endpoint} not in Capsule.mk");
    let before = services().len();
    assert!(announce());
    assert_eq!(services()[before..], [(SERVICE_NAME.to_vec(), SERVICE_PORT)]);
}

#[test]
fn the_compositor_finds_bga_by_the_name_it_announces() {
    let name = core::str::from_utf8(SERVICE_NAME).unwrap();
    assert!(DISPLAYS.contains(&name), "{name} not on the kernel's display list");
}
