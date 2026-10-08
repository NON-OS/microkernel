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

//! The invalidations the kernel sends, each to every unit: the device table
//! is shared, so a change to it is stale in every unit's caches, and a
//! command for a device behind another unit is a harmless no-op.

use super::command::{invalidate_all, invalidate_devtab_entry, invalidate_domain_pages, Command};
use super::error::AmdViError;
use super::regs;
use super::submit::submit;
use super::units::units;

/// The same batch on every unit; every unit is tried, the first error kept.
pub(super) fn submit_all(batch: &[Command]) -> Result<(), AmdViError> {
    let mut outcome = Ok(());
    for index in 0..units().len() {
        outcome = outcome.and(submit(index, batch));
    }
    outcome
}

pub fn flush_device(device_id: u16) -> Result<(), AmdViError> {
    submit_all(&[invalidate_devtab_entry(device_id)])
}

pub fn flush_domain(domain: u16) -> Result<(), AmdViError> {
    submit_all(&[invalidate_domain_pages(domain)])
}

/// Everything a unit may hold from firmware's tables, right after it is
/// enabled with the kernel's, as Linux amd_iommu_flush_all_caches: one
/// INVALIDATE_IOMMU_ALL where EFR.IASup says the unit takes it, otherwise
/// every device table entry and every domain id the kernel hands out.
pub(super) fn flush_everything(index: usize, domains: u16) -> Result<(), AmdViError> {
    let unit = units().get(index).ok_or(AmdViError::NotPresent)?;
    let efr = unit.read64(regs::EXT_FEATURE).unwrap_or(0);
    if regs::invalidate_all_supported(efr) {
        return submit(index, &[invalidate_all()]);
    }
    let mut batch = [[0u32; 4]; 64];
    for first in (0..=u16::MAX as u32).step_by(batch.len()) {
        for (i, slot) in batch.iter_mut().enumerate() {
            *slot = invalidate_devtab_entry((first as usize + i) as u16);
        }
        submit(index, &batch)?;
    }
    for domain in 0..domains {
        submit(index, &[invalidate_domain_pages(domain)])?;
    }
    Ok(())
}
