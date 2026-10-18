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

//! Which mode to run in: the one the firmware already set, when it is a
//! 32-bit mode the device can hold, as UEFI's GOP driver leaves it on VMware
//! and with OVMF; else the default, or a smaller one VRAM can hold.

use crate::constants::mode::{BYTES_PER_PIXEL, DEFAULT_MODE, FALLBACK_MODES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max: (u32, u32),
    pub vram: u32,
}

pub fn choose(firmware: Option<(u32, u32)>, lim: Limits) -> Option<(u32, u32)> {
    let fits = |&(w, h): &(u32, u32)| {
        let bytes = w as u64 * h as u64 * BYTES_PER_PIXEL as u64;
        w > 0 && h > 0 && w <= lim.max.0 && h <= lim.max.1 && bytes <= lim.vram as u64
    };
    if let Some(m) = firmware.filter(fits) {
        return Some(m);
    }
    core::iter::once(DEFAULT_MODE).chain(FALLBACK_MODES).find(fits)
}
