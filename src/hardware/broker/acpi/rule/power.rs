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

//! The FADT's power control registers as address ranges.
//!
//! PM1a and PM1b control take SLP_TYP and SLP_EN, the sleep control register
//! does on a hardware-reduced machine, and the reset register resets it. The
//! kernel writes them itself, on its own path to soft-off and reboot, after
//! the ZeroState wipe; an OperationRegion or a PIO grant over one would let a
//! capsule turn the machine off or reset it with RAM unwiped. Each register
//! is a Generic Address Structure; this turns one into the bytes it covers.

/// A GAS address space: system memory.
pub const SPACE_MEMORY: u8 = 0x00;
/// A GAS address space: system I/O.
pub const SPACE_IO: u8 = 0x01;

/// One power control register: its space and the [start, end) it covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerReg {
    pub space: u8,
    pub start: u64,
    pub end: u64,
}

/// The register a GAS names, or `None` for an absent one (address zero) or
/// one in another space. A register of no stated width is taken as one byte;
/// the span runs to the last byte its bit field reaches.
pub fn power_reg(space: u8, address: u64, bit_offset: u8, bit_width: u8) -> Option<PowerReg> {
    if address == 0 || (space != SPACE_MEMORY && space != SPACE_IO) {
        return None;
    }
    let bits = u64::from(bit_offset) + u64::from(bit_width.max(1));
    let end = address.checked_add(bits.div_ceil(8))?;
    Some(PowerReg { space, start: address, end })
}

/// Whether [base, end) in `space` touches any of `regs`.
pub fn touches<'a>(
    regs: impl IntoIterator<Item = &'a PowerReg>,
    space: u8,
    base: u64,
    end: u64,
) -> bool {
    regs.into_iter().any(|r| r.space == space && r.start < end && base < r.end)
}
