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

//! One page of a PCI function's memory BAR, mapped uncached into the kernel
//! for the length of one closure and unmapped after it: the broker's read of
//! a virtio device's ACCESS_PLATFORM bit before the device is confined
//! (hardware/broker/confine/virtio.rs, K5). The device mapping lives here,
//! in the PCI driver, as map_ecam_bus's does, so the broker's gate on kernel
//! device mappings finds them in the driver tree.

use crate::memory::addr::PhysAddr;
use crate::memory::mmio::{map_device_memory, unmap_mmio};

const PAGE: usize = 4096;

/// `f` run on the kernel VA the page at physical `page` is mapped at, the
/// page unmapped once it returns; None when the page cannot be mapped.
pub fn with_bar_page<R>(page: u64, f: impl FnOnce(u64) -> R) -> Option<R> {
    let va = map_device_memory(PhysAddr::new(page), PAGE).ok()?;
    let r = f(va.as_u64());
    let _ = unmap_mmio(va);
    Some(r)
}
