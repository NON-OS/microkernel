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

//! Folding every battery and adapter into the one word the kernel keeps
//! (src/hardware/broker/acpi/power_report.rs, whose bits these are). Pure,
//! proved in nonos_aml_proofs against a modelled laptop's tables.

use super::state::Power;
use nonos_aml::power::percent;

pub const BATTERY_PRESENT: u64 = 1 << 8;
pub const CHARGING: u64 = 1 << 9;
pub const DISCHARGING: u64 = 1 << 10;
pub const AC_ONLINE: u64 = 1 << 11;
pub const AC_KNOWN: u64 = 1 << 12;
pub const UNKNOWN_PERCENT: u64 = 0xFF;

/// The charge over all present batteries whose numbers are known, weighted
/// by their capacities; flags from any of them.
pub fn word(p: &Power) -> u64 {
    let (mut left, mut full, mut flags) = (0u64, 0u64, 0u64);
    for b in p.batteries.iter().filter(|b| b.present) {
        flags |= BATTERY_PRESENT;
        let (Some(i), Some(s)) = (&b.info, &b.status) else { continue };
        flags |=
            if s.charging { CHARGING } else { 0 } | if s.discharging { DISCHARGING } else { 0 };
        if let (Some(pc), Some(cap)) = (percent(i, s), i.last_full.or(i.design_capacity)) {
            left += u64::from(pc) * u64::from(cap);
            full += u64::from(cap);
        }
    }
    let pc = if full == 0 { UNKNOWN_PERCENT } else { left / full };
    let ac = p
        .adapters
        .iter()
        .filter_map(|a| a.1)
        .fold(None, |acc: Option<bool>, on| Some(acc.unwrap_or(false) || on));
    let ac_bits = match ac {
        Some(true) => AC_KNOWN | AC_ONLINE,
        Some(false) => AC_KNOWN,
        None => 0,
    };
    pc | flags | ac_bits
}
