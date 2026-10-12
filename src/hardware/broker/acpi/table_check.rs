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

//! Whether a copied firmware table may go to the ACPI capsule. Pure:
//! `kernel_proofs` mounts this file as it is.

/// The ACPI capsule never needs a table larger than this, and a length
/// field past it is corruption, not a table.
pub(super) const MAX_TABLE: usize = 4 << 20;
/// Every table begins with a signature and a length (ACPI 6.5, 5.2.6;
/// the FACS too, 5.2.10).
pub(super) const HEADER: usize = 36;
pub(super) const FACS_HEADER: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    Absent,
    /// The table's address is outside the kernel's direct map.
    Unmapped,
    /// The length field is smaller than the header or larger than any table.
    Length,
    /// The bytes at the address carry another signature.
    Signature,
    /// The bytes do not sum to zero (5.2.6), so firmware's table is not what
    /// firmware wrote; the capsule would run AML no one wrote.
    Checksum,
}

/// The length the header at the start of `head` declares, if plausible.
/// The FACS has no checksum and only a signature and length before its
/// fields, so its minimum is shorter.
pub(super) fn declared_length(head: &[u8], sig: [u8; 4]) -> Result<usize, TableError> {
    let min = if sig == *b"FACS" { FACS_HEADER } else { HEADER };
    if head.len() < FACS_HEADER {
        return Err(TableError::Length);
    }
    if head[..4] != sig {
        return Err(TableError::Signature);
    }
    let len = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    if len < min || len > MAX_TABLE {
        return Err(TableError::Length);
    }
    Ok(len)
}

/// A whole table, checked: the signature asked for, its own length, and,
/// for every table but the FACS, a checksum byte that makes the sum zero.
pub(super) fn verify(bytes: &[u8], sig: [u8; 4]) -> Result<(), TableError> {
    if declared_length(bytes, sig)? != bytes.len() {
        return Err(TableError::Length);
    }
    if sig != *b"FACS" && bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b)) != 0 {
        return Err(TableError::Checksum);
    }
    Ok(())
}
