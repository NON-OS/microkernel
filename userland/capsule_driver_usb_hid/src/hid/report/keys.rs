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

//! A report-protocol keyboard report, and a media key report, as the boot
//! keyboard report the keyboard state takes: modifiers and up to six keys.
//! A media key comes out as the keyboard page usage the keycode map gives
//! its code, so it is held, repeated and released as any key.

use crate::descriptors::report::{Keys, Media, MEDIA_KEYS};

use super::super::keyboard::BootReport;
use super::read::{flag, holds, value};

/// None when the report is too short for any of its fields.
pub fn keys_report(k: &Keys, body: &[u8]) -> Option<BootReport> {
    let mut modifiers = 0u8;
    for (i, &at) in k.mods.iter().enumerate() {
        modifiers |= u8::from(flag(body, at)?) << i;
    }
    let s = k.slots?;
    let mut keys = [0u8; 6];
    for (i, key) in keys.iter_mut().enumerate().take(usize::from(s.count)) {
        let v = value(body, s.at + i as u16 * u16::from(s.size), s.size, s.min)?;
        // A value outside the logical range names no key.
        let usage = (s.min..=s.max).contains(&v).then(|| i64::from(k.first) + v - s.min);
        *key = usage.and_then(|u| u8::try_from(u).ok()).unwrap_or(0);
    }
    Some(BootReport { modifiers, keys })
}

pub fn media_report(m: &Media, body: &[u8]) -> Option<BootReport> {
    let mut keys = [0u8; 6];
    let mut n = 0;
    for (i, &(_, key)) in MEDIA_KEYS.iter().enumerate() {
        let in_slot = match (m.slots, m.values[i]) {
            (Some(s), Some(v)) => holds(body, s.at, s.size, s.count, s.min, v)?,
            _ => false,
        };
        if flag(body, m.bits[i])? || in_slot {
            keys[n] = key;
            n += 1;
        }
    }
    Some(BootReport { modifiers: 0, keys })
}
