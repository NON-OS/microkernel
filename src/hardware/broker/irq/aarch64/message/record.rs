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

use spin::Mutex;

use crate::arch::aarch64::gic::msi::{Vector, MAX_VECTORS};

/// Message binds at once: one per device, and a capsule drives a few.
pub(super) const MAX_BINDS: usize = 32;

/// One MSI or MSI-X bind and everything taken for it.
#[derive(Clone, Copy)]
pub(in super::super) struct MessageBind {
    /// The grant of vector 0, zero until its slots are taken; vector `i`
    /// is this plus `i`.
    pub(super) base: u64,
    pub(in super::super) pid: u32,
    pub(in super::super) device_id: u64,
    /// The id the MSI controller knows the function by.
    pub(super) msi_id: u32,
    pub(super) msix: bool,
    pub(super) vectors: [Vector; MAX_VECTORS],
    pub(super) count: usize,
    /// Vectors the controller holds for this bind, a bit each.
    pub(super) live: u32,
    /// The doorbell page as the device reaches it, zero until mapped, and
    /// whether that is an IOVA in the capsule's domain.
    pub(super) doorbell: u64,
    pub(super) confined: bool,
}

/// Every message bind, under one lock held across each bind and teardown,
/// which touch the function, the MSI controller and the capsule's domain in
/// an order that must not interleave. Taken before the IOMMU's lock and the
/// controller's, never inside either.
pub(super) static BINDS: Mutex<[Option<MessageBind>; MAX_BINDS]> = Mutex::new([None; MAX_BINDS]);
