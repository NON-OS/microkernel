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

//! A measured ACPI table's line. Only tables that describe the model, the
//! same on every unit of it, are reported with their digest. A table that can
//! hold something of this one unit (MSDM and SLIC carry an operating-system
//! product key) or that changes from boot to boot keeps only its signature
//! and length, so the report never fingerprints a single machine.

use crate::sink::Sink;

/// The record MkAcpiMeasured writes: signature, instance and length (u32
/// big-endian each), then the table's BLAKE3 digest.
pub const RECORD_LEN: usize = 44;

/// The tables whose digest is reported.
pub const HASHED: [&[u8; 4]; 18] = [
    b"DSDT", b"SSDT", b"FACP", b"APIC", b"MCFG", b"HPET", b"DMAR", b"IVRS", b"SRAT", b"SLIT",
    b"ECDT", b"WAET", b"LPIT", b"NHLT", b"PPTT", b"HMAT", b"CEDT", b"TPM2",
];

pub fn hashed(sig: &[u8; 4]) -> bool {
    HASHED.contains(&sig)
}

/// A signature byte as reported: firmware names its tables in capitals and
/// digits; anything else is written as '_'.
fn sig_byte(b: u8) -> u8 {
    if b.is_ascii_uppercase() || b.is_ascii_digit() {
        b
    } else {
        b'_'
    }
}

/// `acpi <SIG> <instance> <length> <digest or ->`, for one record; false for
/// a record of the wrong size.
pub fn line(rec: &[u8], out: &mut Sink) -> bool {
    if rec.len() != RECORD_LEN {
        return false;
    }
    let sig = [rec[0], rec[1], rec[2], rec[3]];
    let word = |o: usize| u32::from_be_bytes([rec[o], rec[o + 1], rec[o + 2], rec[o + 3]]);
    out.bytes(b"acpi ").bytes(&sig.map(sig_byte)).bytes(b" ");
    out.dec(u64::from(word(4))).bytes(b" ").dec(u64::from(word(8))).bytes(b" ");
    if hashed(&sig) {
        for b in &rec[12..44] {
            out.hex(u64::from(*b), 2);
        }
    } else {
        out.bytes(b"-");
    }
    out.bytes(b"\n");
    true
}
