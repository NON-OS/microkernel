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

//! The name the driver is found under, from both sides: the service it
//! registers, as its Capsule.mk declares it, and the compositor's order of
//! asking, which reaches it after virtio-gpu and before GOP, with the one
//! line the lookup logs for each driver it finds.

use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;

use nonos_libc::services;

use super::gfx_order::{first_announced, news, DRIVERS, GFX_SERVICES};
use crate::serve::{announce, SERVICE_NAME, SERVICE_PORT};

#[test]
fn the_service_is_registered_as_driver_bga0_on_the_port_capsule_mk_declares() {
    assert!(announce(), "the registration was refused");
    assert_eq!(services(), [(b"driver.bga0".to_vec(), SERVICE_PORT)]);
    let mk = include_str!("../../../capsule_driver_bga/Capsule.mk");
    let declared = format!("CAPSULE_SERVICE_ENDPOINT := service:{SERVICE_PORT}:driver.bga0\n");
    assert!(mk.contains(&declared), "Capsule.mk declares another service");
    assert!(mk.contains("CAPSULE_HANDLE           := driver.bga0\n"));
}

#[test]
fn the_compositor_asks_for_virtio_gpu_first_and_this_driver_second() {
    assert_eq!(GFX_SERVICES, [&b"driver.virtio_gpu0"[..], SERVICE_NAME]);
    let lookup = |up: &[&[u8]]| {
        let asked = RefCell::new(Vec::new());
        let port = first_announced(|name| {
            asked.borrow_mut().push(name.to_vec());
            up.iter().position(|u| *u == name).map(|i| 4000 + i as u32)
        });
        (port, asked.into_inner())
    };
    let (virtio, bga) = (b"driver.virtio_gpu0".to_vec(), b"driver.bga0".to_vec());
    let both: [&[u8]; 2] = [b"driver.virtio_gpu0", b"driver.bga0"];
    assert_eq!(lookup(&both), (Some((0, 4000)), vec![virtio.clone()]), "bga asked first");
    assert_eq!(lookup(&both[1..]), (Some((1, 4000)), vec![virtio.clone(), bga.clone()]));
    assert_eq!(lookup(&both[..1]), (Some((0, 4000)), vec![virtio.clone()]));
    assert_eq!(lookup(&[]), (None, vec![virtio, bga]), "GOP: neither announced");
}

#[test]
fn the_lookup_names_each_driver_it_finds_anew_and_never_gop() {
    // GOP is the compositor's to name, once it takes it (wait_for_setup):
    // a lookup that finds nothing may come while it still waits for one.
    assert_eq!(DRIVERS, ["virtio-gpu", "bga"]);
    let said = AtomicUsize::new(usize::MAX);
    let asks = [None, None, Some(1), Some(1), Some(0), Some(0), None, Some(1)];
    let lines: Vec<_> = asks.into_iter().map(|found| news(&said, found)).collect();
    let (bga, virtio) = (Some("bga"), Some("virtio-gpu"));
    assert_eq!(lines, [None, None, bga, None, virtio, None, None, bga], "not one line a find");
}
