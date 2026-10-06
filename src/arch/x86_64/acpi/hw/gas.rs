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

//! Generic Address Structure access (ACPI 6.5 section 5.2.3.2).
//!
//! A GAS names a register by address space, address, bit width, bit offset
//! and access size. The rules for turning that into port or memory cycles
//! follow ACPICA's `acpi_hw_read` / `acpi_hw_write`, which is what Linux runs
//! on every PC, so a register that works there is touched the same way here:
//!
//! - A "register" style GAS (bit offset 0, bit width a power of two of at
//!   least 8) is accessed at its bit width; the access size field is not
//!   consulted. Firmware routinely leaves access size at 0 or sets it wrong
//!   for the FADT blocks, and Linux ignores it there.
//! - Otherwise the access size field decides the width, and when that is 0
//!   the width is the smallest power of two covering offset plus width,
//!   narrowed until the address is aligned to it.
//! - Port I/O never goes wider than 32 bits.
//! - Whole access units that lie entirely below the bit offset are skipped
//!   (read as zero, not written); the value is not shifted.
//!
//! Pure: the actual cycles go through a [`RegisterBus`], so the host proofs
//! drive this with a recording bus and the kernel with real ports and MMIO.

/// ACPI address space ids used by the fixed hardware registers.
pub const SPACE_SYSTEM_MEMORY: u8 = 0x00;
pub const SPACE_SYSTEM_IO: u8 = 0x01;
pub const SPACE_PCI_CONFIG: u8 = 0x02;

/// A decoded Generic Address Structure. Not the packed firmware layout: this
/// is the value the hardware code works with after the FADT was decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Gas {
    pub space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

impl Gas {
    pub const fn empty() -> Self {
        Self { space: 0, bit_width: 0, bit_offset: 0, access_size: 0, address: 0 }
    }

    /// Decode the 12 bytes of a firmware GAS.
    pub fn from_bytes(b: &[u8; 12]) -> Self {
        let mut a = [0u8; 8];
        a.copy_from_slice(&b[4..12]);
        Self {
            space: b[0],
            bit_width: b[1],
            bit_offset: b[2],
            access_size: b[3],
            address: u64::from_le_bytes(a),
        }
    }

    /// A port I/O register of `len_bytes` bytes, the shape ACPICA builds from a
    /// legacy 32-bit FADT block address and its length byte.
    pub const fn io(port: u64, len_bytes: u8) -> Self {
        Self {
            space: SPACE_SYSTEM_IO,
            bit_width: len_bytes.saturating_mul(8),
            bit_offset: 0,
            access_size: 0,
            address: port,
        }
    }

    pub const fn is_present(&self) -> bool {
        self.address != 0
    }

    /// The same register `byte_offset` bytes further on, `bit_width` wide.
    /// Used to split a PM1 event block into its status and enable halves and
    /// a GPE block into its per-byte registers.
    pub fn at_offset(&self, byte_offset: u64, bit_width: u8) -> Self {
        Self {
            space: self.space,
            bit_width,
            bit_offset: 0,
            access_size: self.access_size,
            address: self.address.wrapping_add(byte_offset),
        }
    }

    /// Width in bits of each bus cycle used for this register, per ACPICA's
    /// `acpi_hw_get_access_bit_width` with a 64-bit ceiling.
    pub fn access_bits(&self) -> u8 {
        let mut bits: u8 =
            if self.bit_offset == 0 && self.bit_width >= 8 && self.bit_width.is_power_of_two() {
                self.bit_width
            } else if (1..=4).contains(&self.access_size) {
                1u8 << (self.access_size + 2)
            } else {
                let total = self.bit_offset as u16 + self.bit_width as u16;
                let mut w = total.max(8).next_power_of_two().min(64) as u8;
                if w > 8 {
                    while w > 8 && !self.address.is_multiple_of(w as u64 / 8) {
                        w >>= 1;
                    }
                }
                w
            };
        if bits > 64 {
            bits = 64;
        }
        if self.space == SPACE_SYSTEM_IO && bits > 32 {
            bits = 32;
        }
        bits
    }

    /// Total bits the register spans, offset included. Zero width means the
    /// firmware left the field blank; one access unit is then used.
    fn span_bits(&self) -> u16 {
        let span = self.bit_offset as u16 + self.bit_width as u16;
        if self.bit_width == 0 {
            self.access_bits() as u16
        } else {
            span
        }
    }

    /// Whether this register can be reached by [`gas_read`] / [`gas_write`]:
    /// present, in memory or port space, at most 64 bits, and a port register
    /// whose whole span fits below port 0x10000.
    pub fn is_accessible(&self) -> bool {
        if !self.is_present() || self.span_bits() > 64 {
            return false;
        }
        match self.space {
            SPACE_SYSTEM_MEMORY => true,
            SPACE_SYSTEM_IO => {
                let bytes = (self.span_bits() as u64).div_ceil(8);
                self.address.checked_add(bytes).map(|end| end <= 0x1_0000).unwrap_or(false)
            }
            _ => false,
        }
    }
}

/// The bus a GAS is accessed over. `bits` is always 8, 16, 32 or 64 (64 only
/// for memory).
pub trait RegisterBus {
    fn read_io(&mut self, port: u16, bits: u8) -> u32;
    fn write_io(&mut self, port: u16, bits: u8, value: u32);
    /// None when the physical address cannot be mapped.
    fn read_mem(&mut self, phys: u64, bits: u8) -> Option<u64>;
    /// False when the physical address cannot be mapped.
    fn write_mem(&mut self, phys: u64, bits: u8, value: u64) -> bool;
}

fn mask(bits: u8) -> u64 {
    if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    }
}

fn read_unit<B: RegisterBus>(bus: &mut B, g: &Gas, addr: u64, bits: u8) -> Option<u64> {
    match g.space {
        SPACE_SYSTEM_MEMORY => bus.read_mem(addr, bits),
        SPACE_SYSTEM_IO => Some(bus.read_io(addr as u16, bits) as u64),
        _ => None,
    }
}

fn write_unit<B: RegisterBus>(bus: &mut B, g: &Gas, addr: u64, bits: u8, v: u64) -> bool {
    match g.space {
        SPACE_SYSTEM_MEMORY => bus.write_mem(addr, bits, v),
        SPACE_SYSTEM_IO => {
            bus.write_io(addr as u16, bits, v as u32);
            true
        }
        _ => false,
    }
}

/// Read a register. None when the GAS is absent, malformed or in a space
/// this code does not drive.
pub fn gas_read<B: RegisterBus>(bus: &mut B, g: &Gas) -> Option<u64> {
    if !g.is_accessible() {
        return None;
    }
    let width = g.access_bits();
    let mut remaining = g.span_bits();
    let mut skip = g.bit_offset as u16;
    let mut value: u64 = 0;
    let mut index: u32 = 0;
    while remaining > 0 {
        let shift = index * width as u32;
        if shift >= 64 {
            break;
        }
        let unit = if skip >= width as u16 {
            skip -= width as u16;
            0
        } else {
            let addr = g.address.wrapping_add(index as u64 * (width as u64 / 8));
            read_unit(bus, g, addr, width)? & mask(width)
        };
        value |= unit << shift;
        remaining -= remaining.min(width as u16);
        index += 1;
    }
    Some(value)
}

/// Write a register. False when the GAS is absent, malformed or in a space
/// this code does not drive, or when a memory unit could not be mapped.
pub fn gas_write<B: RegisterBus>(bus: &mut B, g: &Gas, value: u64) -> bool {
    if !g.is_accessible() {
        return false;
    }
    let width = g.access_bits();
    let mut remaining = g.span_bits();
    let mut skip = g.bit_offset as u16;
    let mut index: u32 = 0;
    while remaining > 0 {
        let shift = index * width as u32;
        if shift >= 64 {
            break;
        }
        if skip >= width as u16 {
            skip -= width as u16;
        } else {
            let unit = (value >> shift) & mask(width);
            let addr = g.address.wrapping_add(index as u64 * (width as u64 / 8));
            if !write_unit(bus, g, addr, width, unit) {
                return false;
            }
        }
        remaining -= remaining.min(width as u16);
        index += 1;
    }
    true
}
