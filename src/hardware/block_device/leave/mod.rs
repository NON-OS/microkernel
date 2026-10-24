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

//! The leave phase, before power goes (zerostate terminate): every running
//! disk driver asked at once to make its disks safe, NVMe and AHCI with
//! their power-off op (flush, then the controller's shutdown or the disk's
//! standby), the rest with a flush. Each is bounded by its own budget and
//! all of them by the total (rule.rs), so a disk that never answers costs
//! its budget and the power-off goes on. One line per driver says how it
//! ended. An amnesic boot flushes too: the drive's own cache must settle.

mod leg;
mod leg_poll;
mod poll;
mod rule;

use alloc::format;
use alloc::vec::Vec;

pub(crate) use leg::{Leg, Wire};
use rule::{run, Ended, TOTAL_MS};

pub fn power_off_disks() {
    let mut legs: Vec<Leg> = Vec::new();
    legs.extend(crate::hardware::nvme_capsule::leave_leg());
    legs.extend(crate::hardware::ahci_capsule::leave_leg());
    legs.extend(crate::hardware::virtio_blk_capsule::leave_leg());
    legs.extend(crate::hardware::usb_msc_capsule::leave_leg());
    legs.extend(super::wire::leave_legs());
    let budgets: Vec<u64> = legs.iter().map(|l| l.budget_ms).collect();
    let mut round = 0u32;
    let now = || Some(crate::time::timestamp_millis());
    let rest = || {
        crate::services::lifecycle::rest(round);
        round = round.saturating_add(1);
    };
    let ended = run(&budgets, TOTAL_MS, now, |i| legs[i].poll(), rest);
    for (leg, end) in legs.iter().zip(ended) {
        let line = match end {
            Ended::Answered(0) => format!("{} done", leg.name),
            Ended::Answered(status) => format!("{} answered {}", leg.name, status),
            Ended::Gone => format!("{} not running, skipped", leg.name),
            Ended::Missed => {
                format!("{} missed its {} ms budget, skipped", leg.name, leg.budget_ms)
            }
            Ended::NoClock => format!("{} not waited on: the clock cannot be read", leg.name),
        };
        crate::sys::boot_log::ok("POWER", &format!("leave: {line}"));
    }
}
