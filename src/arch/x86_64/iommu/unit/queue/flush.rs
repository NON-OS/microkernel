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

//! The invalidations the kernel queues, each as one batch with its wait.

use super::descriptor::{context_global, iec_global, iotlb_global};
use super::submit::submit;
use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// Context cache, then IOTLB, in the order a table change requires: a stale
/// translation outlives the context entry that produced it.
pub fn flush_caches(unit: &RemapUnit) -> Result<(), VtdError> {
    submit(unit, &[context_global(), iotlb(unit)])
}

/// Every cached translation, after leaf entries changed.
pub fn flush_iotlb(unit: &RemapUnit) -> Result<(), VtdError> {
    submit(unit, &[iotlb(unit)])
}

/// Every cached interrupt remapping entry, after the table changed.
pub fn flush_interrupt_entries(unit: &RemapUnit) -> Result<(), VtdError> {
    submit(unit, &[iec_global()])
}

fn iotlb(unit: &RemapUnit) -> [u64; 2] {
    let word = unit.read64(offsets::CAP);
    iotlb_global(cap::read_drain(word), cap::write_drain(word))
}
