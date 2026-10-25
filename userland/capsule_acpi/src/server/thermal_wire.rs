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

//! THERMAL (7): what the shell shows of the machine's heat. The number of
//! zones (at most 64), then for each its temperature and its _CRT, in
//! tenths of a kelvin (0 when unknown or absent), and its state bits: 1
//! passive, 2 hot, 4 critical, 8 when a fan it lists is running, 16 when
//! its latest reading failed and the temperature shown is the one before.

use crate::boot::Acpi;
use alloc::vec::Vec;
use nonos_aml::thermal::tenths_kelvin;

const MAX_ZONES: usize = 64;

pub fn thermal(acpi: &Acpi) -> (i32, Vec<u8>) {
    let c = &acpi.cooling;
    let zones = &c.zones[..c.zones.len().min(MAX_ZONES)];
    let mut v = Vec::with_capacity(1 + zones.len() * 5);
    v.push(zones.len() as u8);
    for z in zones {
        let dk = |t: Option<i32>| t.map_or(0, |t| tenths_kelvin(t).min(0xFFFF) as u16);
        let fan = z.levels.iter().flatten().any(|&f| c.fans[f].on == Some(true));
        let v_ = &z.w.verdict;
        let bits = u8::from(v_.passive) | u8::from(v_.hot) << 1 | u8::from(z.w.critical) << 2;
        v.extend_from_slice(&dk(z.w.temp).to_le_bytes());
        v.extend_from_slice(&dk(z.trips.critical).to_le_bytes());
        v.push(bits | u8::from(fan) << 3 | u8::from(z.w.unread) << 4);
    }
    (0, v)
}
