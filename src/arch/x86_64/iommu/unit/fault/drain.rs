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

use super::budget::Budget;
use super::count::count;
use super::log::{log_hidden, log_overflow, log_record};
use super::status::{clear_status, has_faults, overflowed};
use super::take::take_fault;
use crate::arch::x86_64::iommu::regs::cap;
use crate::arch::x86_64::iommu::unit::probe::UnitInfo;
use crate::arch::x86_64::iommu::unit::report::units;

/// Drain every pending fault record and return how many.
///
/// A denial the operator cannot see is indistinguishable from a hang, so this
/// is what makes enforcement diagnosable: a device that stops working under
/// translation names itself here. Every record is cleared, whether or not the
/// budget lets it print, since a full set of records stops the unit recording.
pub fn drain_faults() -> usize {
    let mut budget = Budget::default();
    let drained = units().iter().map(|info| drain_unit(info, &mut budget)).sum();
    count(drained);
    if budget.hidden > 0 {
        log_hidden(budget.hidden);
    }
    drained
}

fn drain_unit(info: &UnitInfo, budget: &mut Budget) -> usize {
    if !has_faults(&info.unit) {
        return 0;
    }
    if overflowed(&info.unit) {
        log_overflow(info.unit.base_pa());
    }

    let mut drained = 0;
    for index in 0..cap::fault_recording_count(info.cap) as usize {
        if let Some(record) = take_fault(&info.unit, info.cap, index) {
            if budget.admit() {
                log_record(&record);
            }
            drained += 1;
        }
    }
    // SAFETY: eK@nonos.systems - every record was taken above, so no pending
    // record is left with nothing advertising it.
    unsafe {
        clear_status(&info.unit);
    }
    drained
}
