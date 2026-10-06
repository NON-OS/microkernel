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

//! Putting one stopped unit in service with the kernel's device table, in
//! the order of Linux amd/init.c, early_enable_iommu: device table, command
//! buffer, event log, then IOMMU_EN, then every cache flushed.

use super::control::{idle, CMD_BUF_EN, CONTROL, EVENT_LOG_EN, IOMMU_EN};
use super::error::AmdViError;
use super::flush::flush_everything;
use super::log_ring::attach_log;
use super::mmio::Unit;
use super::regs;
use super::ring::attach_ring;

/// Exclusion Base: a range firmware may have set that every device reaches
/// untranslated. Cleared, so the kernel's tables are the whole story.
const EXCLUSION_BASE: usize = 0x0020;

/// # Safety
/// `table` must be the shared device table, alive for the kernel's life,
/// with every enumerated device's entry already written.
pub(super) unsafe fn enable_unit(index: usize, unit: &Unit, table: u64) -> Result<(), AmdViError> {
    let control = unit.read64(CONTROL).ok_or(AmdViError::RegistersUnmappable)?;
    let idle = idle(control);
    // SAFETY: eK@nonos.systems - the unit is stopped before any base moves,
    // the pages it is pointed at are the kernel's for good, and the device
    // table denies every device not enumerated.
    unsafe {
        unit.write64(CONTROL, idle);
        unit.write64(EXCLUSION_BASE, 0);
        unit.write64(regs::DEV_TABLE_BASE, regs::dev_table_base(table, regs::DEV_TABLE_PAGES));
        attach_ring(index, unit)?;
        attach_log(index, unit)?;
        unit.write64(CONTROL, idle | CMD_BUF_EN | EVENT_LOG_EN);
        unit.write64(CONTROL, idle | CMD_BUF_EN | EVENT_LOG_EN | IOMMU_EN);
    }
    flush_everything(index, super::domain::MAX_DOMAINS as u16)
}
