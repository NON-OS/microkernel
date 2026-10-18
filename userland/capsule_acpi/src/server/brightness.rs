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

//! The brightness wire, which the graphics drivers' native backlight is to
//! serve too, so the shell can choose one per panel. BRIGHTNESS (3): the
//! number of panels (at most 4), then for each its identity (u32, the
//! output's _ADR), its level now (0xFF unknown), and its levels, a count
//! and the percentages. SET_BRIGHTNESS (4, percent[, panel u32]): one
//! panel, or every panel when none is named, to its level nearest the
//! percentage asked.

use super::protocol::{EBUSY, EINVAL, EIO, ENOENT};
use crate::backlight::firmware_drives;
use crate::backlight::set_all;
use crate::boot::Acpi;
use alloc::vec::Vec;

const UNKNOWN: u8 = 0xFF;
const MAX_PANELS: usize = 4;

pub fn get(acpi: &Acpi) -> (i32, Vec<u8>) {
    let outputs = &acpi.backlight.outputs;
    if outputs.is_empty() {
        return (ENOENT, Vec::new());
    }
    let shown = &outputs[..outputs.len().min(MAX_PANELS)];
    let mut v = Vec::new();
    v.push(shown.len() as u8);
    for o in shown {
        v.extend_from_slice(&o.panel.to_le_bytes());
        v.push(o.current.unwrap_or(UNKNOWN));
        v.push(o.levels.steps.len() as u8);
        v.extend_from_slice(&o.levels.steps);
    }
    (0, v)
}

pub fn set(acpi: &mut Acpi, req: &[u8]) -> (i32, Vec<u8>) {
    let panel = req.get(2..6).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let known = |p: u32| acpi.backlight.outputs.iter().any(|o| o.panel == p);
    let code = match req.get(1).copied() {
        None => EINVAL,
        Some(p) if p > 100 => EINVAL,
        Some(_) if acpi.backlight.outputs.is_empty() => ENOENT,
        Some(_) if panel.is_some_and(|p| !known(p)) => ENOENT,
        Some(_) if !firmware_drives(acpi, panel) => EBUSY,
        Some(p) => {
            let Acpi { backlight, aml, host, .. } = acpi;
            if set_all(backlight, aml, host, p, panel) {
                0
            } else {
                EIO
            }
        }
    };
    (code, Vec::new())
}
