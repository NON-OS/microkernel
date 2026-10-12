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

//! Register blocks the FADT names, reduced to what the event layer reaches:
//! a run of I/O ports. A Generic Address Structure (ACPI 6.5, 5.2.3.2)
//! naming memory instead is reported, not guessed at.

/// The System I/O address space id (table 5.1); every other space is
/// refused, SystemMemory (0) included.
pub const SYSTEM_IO: u8 = 1;

/// A block of `len` bytes of I/O ports from `port`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub port: u16,
    pub len: u8,
}

/// A Generic Address Structure: 12 bytes at `at`, or None past the table.
pub fn gas(t: &[u8], at: usize) -> Option<(u8, u64)> {
    let b = t.get(at..at.checked_add(12)?)?;
    let mut address = [0u8; 8];
    address.copy_from_slice(&b[4..12]);
    Some((b[0], u64::from_le_bytes(address)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockError {
    /// The block lives in memory, which this layer does not reach.
    Memory,
    /// Past the 64 KiB of ports.
    Range,
}

/// The block from the 64-bit X_ field when it gives an address, else from
/// the 32-bit field (5.2.9: X_ is preferred when nonzero). A zero address or
/// length means the platform has no such block.
pub fn block(x: Option<(u8, u64)>, legacy: u32, len: u8) -> Result<Option<Block>, BlockError> {
    let (space, address) = match x {
        Some((space, address)) if address != 0 => (space, address),
        _ => (SYSTEM_IO, u64::from(legacy)),
    };
    if address == 0 || len == 0 {
        return Ok(None);
    }
    if space != SYSTEM_IO {
        return Err(BlockError::Memory);
    }
    let end = address + u64::from(len);
    if end > 0x1_0000 {
        return Err(BlockError::Range);
    }
    Ok(Some(Block { port: address as u16, len }))
}
