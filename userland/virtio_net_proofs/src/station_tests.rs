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

//! The driver never takes the device's MAC feature, on either transport, so
//! the address a hypervisor or a card hands out is never what it transmits.

use crate::constants::{LEGACY_WANTED, MODERN_WANTED, VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS};

#[test]
fn neither_transport_takes_the_device_mac() {
    assert_eq!(LEGACY_WANTED & (1u32 << VIRTIO_NET_F_MAC), 0);
    assert_eq!(MODERN_WANTED & (1u64 << VIRTIO_NET_F_MAC), 0);
}

#[test]
fn link_status_is_still_taken() {
    assert_ne!(LEGACY_WANTED & (1u32 << VIRTIO_NET_F_STATUS), 0);
    assert_ne!(MODERN_WANTED & (1u64 << VIRTIO_NET_F_STATUS), 0);
}

#[test]
fn the_driver_draws_its_address() {
    let station = include_str!("../../capsule_driver_virtio_net/src/setup/station.rs");
    assert!(station.contains("crypto_random"));
    assert!(station.contains("apply(&mut mac)"));
    for path in [
        include_str!("../../capsule_driver_virtio_net/src/setup/sequence.rs"),
        include_str!("../../capsule_driver_virtio_net/src/setup/modern/run.rs"),
    ] {
        assert!(path.contains("station::draw()?"), "a bring-up path takes its MAC elsewhere");
    }
}
