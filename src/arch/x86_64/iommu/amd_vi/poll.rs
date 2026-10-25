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

//! Reading each unit's event log into the console. The VT-d fault poll calls
//! this on the same timer pace, with the same per-poll budget.

use super::event::{self, Event};
use super::log_line::{log_event, log_hidden};
use super::log_ring::{restart_log, LOGS};
use super::regs;
use super::units::{is_enforcing, units};
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::arch::x86_64::iommu::unit::fault::Budget;

pub fn drain_events() {
    if !is_enforcing() {
        return;
    }
    let mut budget = Budget::default();
    for (index, unit) in units().iter().enumerate() {
        let phys = LOGS[index].load(core::sync::atomic::Ordering::Acquire);
        let Ok(words) = Frame::at(phys) else { continue };
        let (Some(head), Some(tail)) =
            (unit.read64(regs::EVENT_HEAD), unit.read64(regs::EVENT_TAIL))
        else {
            continue;
        };
        let (mut head, tail) = (regs::ring_index(head), regs::ring_index(tail));
        while head != tail {
            let at = head as usize * 2;
            let (Ok(low), Ok(high)) = (words.read(at), words.read(at + 1)) else { break };
            let raw: Event = event::from_words(low, high);
            // A zero code is an entry the unit has not finished writing;
            // it is read again next poll rather than logged as nothing.
            if event::code(raw) == 0 {
                break;
            }
            if event::code(raw) == event::IO_PAGE_FAULT {
                crate::memory::iommu::note_fault(event::device_id(raw));
            }
            if budget.admit() {
                log_event(unit.base_pa(), raw);
            }
            if words.write(at, 0).and(words.write(at + 1, 0)).is_err() {
                break;
            }
            head = regs::ring_next(head);
        }
        // SAFETY: eK@nonos.systems - the head only hands consumed slots back
        // to the unit; it maps nothing.
        unsafe { unit.write64(regs::EVENT_HEAD, regs::ring_offset(head)) };
        if unit.read64(regs::STATUS).unwrap_or(0) & regs::STATUS_EVENT_OVERFLOW != 0 {
            restart_log(index, unit);
            log_hidden(0, true);
        }
    }
    if budget.hidden > 0 {
        log_hidden(budget.hidden, false);
    }
}
