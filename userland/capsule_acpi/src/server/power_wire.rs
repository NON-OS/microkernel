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

//! POWER (6): the power wire the shell's battery indicator reads. Batteries
//! counted and present; state bits (1 charging, 2 discharging, 4 critical);
//! the charge (0xFF unknown); the summed rate and minutes left, until empty
//! or full (0xFFFFFFFF unknown); AC (0 offline, 1 online, 0xFF unknown);
//! and the unit, 1 for mA and mAh, 0 for mW and mWh. Then the Type-C
//! ports (`typec_wire`).

use crate::boot::Acpi;
use crate::power::Power;
use alloc::vec::Vec;
use nonos_aml::power::{minutes_left, BatteryInfo, BatteryStatus};

const UNKNOWN: u32 = u32::MAX;

fn summed(p: &Power) -> (BatteryInfo, BatteryStatus) {
    let (mut info, mut st) = (BatteryInfo::default(), BatteryStatus::default());
    let add = |a: Option<u32>, b: Option<u32>| Some(a.unwrap_or(0).saturating_add(b?));
    for b in p.batteries.iter().filter(|b| b.present) {
        let (Some(i), Some(s)) = (&b.info, &b.status) else { continue };
        info.milliamp = i.milliamp;
        info.last_full = add(info.last_full, i.last_full.or(i.design_capacity));
        st.rate = add(st.rate, s.rate);
        st.remaining = add(st.remaining, s.remaining);
        st.charging |= s.charging;
        st.discharging |= s.discharging;
        st.critical |= s.critical;
    }
    (info, st)
}

pub fn power(acpi: &Acpi) -> (i32, Vec<u8>) {
    let p = &acpi.power;
    let word = crate::power::word(p);
    let (info, st) = summed(p);
    let state = u8::from(st.charging) | u8::from(st.discharging) << 1 | u8::from(st.critical) << 2;
    let ac = match (word & (1 << 12) != 0, word & (1 << 11) != 0) {
        (true, on) => u8::from(on),
        (false, _) => 0xFF,
    };
    let mut v = Vec::with_capacity(16);
    v.push(p.batteries.len().min(255) as u8);
    v.push(p.batteries.iter().filter(|b| b.present).count().min(255) as u8);
    v.extend_from_slice(&[state, (word & 0xFF) as u8]);
    v.extend_from_slice(&st.rate.unwrap_or(UNKNOWN).to_le_bytes());
    v.extend_from_slice(&minutes_left(&info, &st).unwrap_or(UNKNOWN).to_le_bytes());
    v.extend_from_slice(&[ac, u8::from(info.milliamp)]);
    super::typec_wire::append(acpi, &mut v);
    (0, v)
}
