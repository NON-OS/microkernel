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

//! Kernel-mediated PIO write. Width selects which architectural
//! `out` instruction fires; the high bits of the supplied `u32`
//! are discarded for narrower widths.

use super::super::types::{PioError, PioWidth};
use super::raw::{out16, out32, out8};
use super::reset_rule::resets_machine;
use super::resolve::resolve;
use crate::hardware::broker::claim;

pub fn write(
    pid: u32,
    grant_id: u64,
    port_offset: u16,
    width: PioWidth,
    value: u32,
) -> Result<(), PioError> {
    let (g, _held) = resolve(pid, grant_id, port_offset, width)?;
    let port = g.port_base + port_offset;
    if resets_machine(port, width as u8, value) {
        return Err(PioError::ReservedPort);
    }
    // SAFETY: the port lies in the grant's window, bounds checked by resolve.
    let write = || unsafe {
        match width {
            PioWidth::U8 => out8::raw_out8(port, value as u8),
            PioWidth::U16 => out16::raw_out16(port, value as u16),
            PioWidth::U32 => out32::raw_out32(port, value),
        }
    };
    // Never after an exit dropped the claim and stopped the device (L69).
    claim::while_live(pid, g.device_id, g.claim_epoch, write).ok_or(PioError::NotClaimed)
}
