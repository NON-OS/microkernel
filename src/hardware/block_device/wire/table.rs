// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Every capsule serving the block wire, in the order the block layer asks
//! them. A new block driver appends its mirror's endpoint here.

use core::sync::atomic::AtomicU64;

use super::endpoint::{Endpoint, MAX_UNITS};

static DRIVERS: &[&Endpoint] = &[
    #[cfg(feature = "nonos-capsule-driver-sdhci")]
    &crate::hardware::sdhci_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-ufs")]
    &crate::hardware::ufs_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-storvsc")]
    &crate::hardware::storvsc_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-piix-ide")]
    &crate::hardware::piix_ide_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-virtio-scsi")]
    &crate::hardware::virtio_scsi_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-pvscsi")]
    &crate::hardware::pvscsi_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-buslogic")]
    &crate::hardware::buslogic_capsule::ENDPOINT,
    #[cfg(feature = "nonos-capsule-driver-mpt")]
    &crate::hardware::mpt_capsule::ENDPOINT,
];

/// How many endpoints this kernel carries.
pub(super) fn count() -> u8 {
    DRIVERS.len() as u8
}

pub(super) fn endpoint(index: u8) -> Option<&'static Endpoint> {
    DRIVERS.get(index as usize).copied()
}

/// What the log calls endpoint `index`'s disk.
pub(super) fn shown(index: u8) -> &'static str {
    endpoint(index).map_or("an unknown block-wire driver", |e| e.shown)
}

/// Where the block layer keeps the hash of its last line for a unit. Past
/// the table (never asked: the rows only count what it holds) the lines
/// share one slot.
pub(super) fn said(index: u8, unit: u16) -> &'static AtomicU64 {
    static PAST: AtomicU64 = AtomicU64::new(0);
    match endpoint(index) {
        Some(e) if (unit as usize) < MAX_UNITS => &e.slot.said[unit as usize],
        _ => &PAST,
    }
}
