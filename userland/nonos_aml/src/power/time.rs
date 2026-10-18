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

//! The charge, and the time to empty and to full, from _BST's rate and
//! remaining capacity and the battery's last full charge (ACPI 6.5,
//! 10.2.2.11). The rate is in the same unit as the capacities, mA against
//! mAh or mW against mWh, so the ratio is hours whichever the battery
//! reports in.

use super::info::BatteryInfo;
use super::status::BatteryStatus;

/// Minutes until empty while discharging, until full while charging;
/// None when idle, or when the rate or a capacity is unknown or zero.
pub fn minutes_left(info: &BatteryInfo, status: &BatteryStatus) -> Option<u32> {
    let rate = u64::from(status.rate.filter(|r| *r > 0)?);
    let left = u64::from(status.remaining?);
    let amount = if status.discharging {
        left
    } else if status.charging {
        let full = info.last_full.or(info.design_capacity).filter(|f| *f > 0)?;
        u64::from(full).saturating_sub(left)
    } else {
        return None;
    };
    u32::try_from(amount * 60 / rate).ok()
}

/// The charge as a percentage of the last full charge (else of the design
/// capacity), 0 to 100; None when either number is unknown or zero.
pub fn percent(info: &BatteryInfo, status: &BatteryStatus) -> Option<u8> {
    let full = info.last_full.filter(|f| *f > 0).or(info.design_capacity.filter(|d| *d > 0))?;
    let left = status.remaining?;
    Some((u64::from(left.min(full)) * 100 / u64::from(full)) as u8)
}
