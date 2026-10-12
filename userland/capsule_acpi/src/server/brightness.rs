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

//! BRIGHTNESS (3) and SET_BRIGHTNESS (4, percent): the backlight over the
//! capsule's port. BRIGHTNESS answers the first output's level now
//! (0xFF when unknown), how many outputs there are, and that output's
//! levels; SET_BRIGHTNESS moves every output to its level nearest the
//! percentage asked.

use super::protocol::{EINVAL, EIO, ENOENT};
use crate::backlight::set_all;
use crate::boot::Acpi;
use alloc::vec::Vec;

const UNKNOWN: u8 = 0xFF;

pub fn get(acpi: &Acpi) -> (i32, Vec<u8>) {
    let Some(o) = acpi.backlight.outputs.first() else { return (ENOENT, Vec::new()) };
    let mut v = Vec::with_capacity(3 + o.levels.steps.len());
    v.push(o.current.unwrap_or(UNKNOWN));
    v.push(acpi.backlight.outputs.len().min(255) as u8);
    v.push(o.levels.steps.len() as u8);
    v.extend_from_slice(&o.levels.steps);
    (0, v)
}

pub fn set(acpi: &mut Acpi, req: &[u8]) -> (i32, Vec<u8>) {
    let code = match req.get(1).copied() {
        None => EINVAL,
        Some(p) if p > 100 => EINVAL,
        Some(_) if acpi.backlight.outputs.is_empty() => ENOENT,
        Some(p) => {
            let Acpi { backlight, aml, host, .. } = acpi;
            if set_all(backlight, aml, host, p) {
                0
            } else {
                EIO
            }
        }
    };
    (code, Vec::new())
}
