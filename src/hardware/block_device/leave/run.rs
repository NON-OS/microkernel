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

//! The leave phase run (mod.rs): every running disk driver's leg, the
//! schedule over them (rule.rs) within `total_ms`, and a line per driver.

use alloc::format;
use alloc::vec::Vec;

use super::leg::Leg;
use super::rule::{run, Ended};

/// Make every disk safe within `total_ms`; on a `restart`, flushed and
/// left spinning.
pub(crate) fn power_off_disks(restart: bool, total_ms: u64) {
    let mut legs: Vec<Leg> = Vec::new();
    legs.extend(crate::hardware::nvme_capsule::leave_leg());
    legs.extend(crate::hardware::ahci_capsule::leave_leg(restart));
    legs.extend(crate::hardware::virtio_blk_capsule::leave_leg());
    legs.extend(crate::hardware::usb_msc_capsule::leave_leg());
    legs.extend(super::super::wire::leave_legs());
    let budgets: Vec<u64> = legs.iter().map(|l| l.budget_ms).collect();
    let mut round = 0u32;
    let now = || Some(crate::time::timestamp_millis());
    let rest = || {
        crate::services::lifecycle::rest(round);
        round = round.saturating_add(1);
    };
    let ended = run(&budgets, total_ms, now, |i| legs[i].poll(), rest);
    for (leg, end) in legs.iter().zip(ended) {
        let line = match end {
            Ended::Answered(0) => format!("{} done", leg.name),
            Ended::Answered(status) => format!("{} answered {}", leg.name, status),
            Ended::Gone => format!("{} not running, skipped", leg.name),
            Ended::Missed => {
                format!(
                    "{} missed its {} ms budget, skipped",
                    leg.name,
                    leg.budget_ms.min(total_ms)
                )
            }
            Ended::NoClock => format!("{} not waited on: the clock cannot be read", leg.name),
        };
        crate::sys::boot_log::ok("POWER", &format!("leave: {line}"));
    }
}
