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

//! Writing one interrupt remapping entry where every unit will see it.

use super::irte::Irte;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::tables::publish::publish;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::queue::{iec_index, submit};
use crate::arch::x86_64::iommu::unit::report::units;

/// Present bit last on a write and first on a clear, so the unit never reads
/// a present entry with half its fields; then the cached copy is dropped on
/// every unit, since the table is shared.
pub(super) fn write(table_phys: u64, index: u16, entry: Irte) -> Result<(), VtdError> {
    let entries = entries_mut(table_phys)?;
    let at = index as usize * 2;
    if entry[0] == 0 {
        entries[at] = 0;
        entries[at + 1] = 0;
    } else {
        entries[at + 1] = entry[1];
        entries[at] = entry[0];
    }
    publish(table_phys);
    let mut outcome = Ok(());
    for info in units() {
        outcome = outcome.and(submit(&info.unit, &[iec_index(index, 0)]));
    }
    outcome
}
