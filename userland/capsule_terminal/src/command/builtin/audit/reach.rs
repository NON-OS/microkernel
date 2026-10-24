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

//! What a capability mask lets a capsule reach, by the kernel's own gates
//! (`src/syscall/caps/checks`, `src/capabilities/token/types`): the network
//! through `Network`; storage through `FileSystem`, `StoreWrite` and
//! `StreamImport`; devices through `Driver`, `Mmio`, `Irq`, `Dma` and `Pio`.
//! `Admin` stands in for none of them, and `Hardware` and `DeviceEnum`
//! reach no device. The device column is capability bits, not live grants:
//! what the kernel would let the capsule claim, not what it holds now.

use alloc::vec::Vec;

use crate::command::builtin::cap_names::CAP_NAMES;

/// One question asked of a mask, and the capabilities that answer it.
pub struct Area {
    pub label: &'static [u8],
    /// What a holder can do, as the count line says it.
    pub said: &'static [u8],
    pub caps: &'static [&'static [u8]],
}

pub const NETWORK: Area =
    Area { label: b"network", said: b"can reach the network", caps: &[b"Network"] };
pub const STORAGE: Area = Area {
    label: b"storage",
    said: b"can reach storage",
    caps: &[b"FileSystem", b"StoreWrite", b"StreamImport"],
};
pub const DEVICES: Area = Area {
    label: b"devices*",
    said: b"can drive devices*",
    caps: &[b"Driver", b"Mmio", b"Irq", b"Dma", b"Pio"],
};
pub const AREAS: [Area; 3] = [NETWORK, STORAGE, DEVICES];

/// Said under every table that has a device column.
pub const DEVICES_NOTE: &[u8] = b"* devices: capability bits the kernel would honour, not devices \
claimed now; live DMA, MMIO and IRQ grants come in the next release.";

/// The bit for a capability name, from the one table `receipt` reads, so
/// this rule and the capability list cannot name different bits.
pub fn bit(name: &[u8]) -> u64 {
    CAP_NAMES.iter().position(|n| *n == name).map_or(0, |i| 1 << i)
}

/// The capabilities in `caps` that give the area, in the area's order.
pub fn through(caps: u64, area: &Area) -> Vec<&'static [u8]> {
    area.caps.iter().copied().filter(|n| caps & bit(n) != 0).collect()
}

pub fn reaches(caps: u64, area: &Area) -> bool {
    !through(caps, area).is_empty()
}
