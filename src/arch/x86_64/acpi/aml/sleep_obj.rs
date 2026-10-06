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

//! Reading the `\_Sx` sleep type package out of AML bytecode without
//! executing it.
//!
//! `\_S5` (and its siblings) is in practice always a named constant package,
//! `Name (_S5, Package (0x04) { 0x07, 0x07, Zero, Zero })`, sometimes wrapped
//! in an `If` that the firmware never changes at run time, and occasionally
//! `Method (_S5) { Return (Package () { ... }) }`. Both forms are recognised
//! here. Anything else (elements that are names or expressions) yields None:
//! the value cannot be known without an interpreter, and a guessed SLP_TYP
//! puts the machine in some other sleep state.
//!
//! Element decoding follows ACPICA's `acpi_get_sleep_type_data`: one element
//! carries SLP_TYPa in bits 0..7 and SLP_TYPb in bits 8..15; two or more give
//! SLP_TYPa then SLP_TYPb. Only the low three bits reach the hardware.

const NAME_OP: u8 = 0x08;
const METHOD_OP: u8 = 0x14;
const PACKAGE_OP: u8 = 0x12;
const RETURN_OP: u8 = 0xA4;
const ROOT_CHAR: u8 = 0x5C;
const ZERO_OP: u8 = 0x00;
const ONE_OP: u8 = 0x01;
const ONES_OP: u8 = 0xFF;
const BYTE_PREFIX: u8 = 0x0A;
const WORD_PREFIX: u8 = 0x0B;
const DWORD_PREFIX: u8 = 0x0C;
const QWORD_PREFIX: u8 = 0x0E;

/// Decoded `\_Sx` values, already masked to the 3-bit SLP_TYP field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SleepPackage {
    pub slp_typ_a: u8,
    pub slp_typ_b: u8,
}

/// PkgLength (ACPI 6.5 section 20.2.4): returns (value, bytes used).
fn pkg_length(aml: &[u8], at: usize) -> Option<(usize, usize)> {
    let lead = *aml.get(at)?;
    let follow = (lead >> 6) as usize;
    if follow == 0 {
        return Some(((lead & 0x3F) as usize, 1));
    }
    let mut value = (lead & 0x0F) as usize;
    for i in 0..follow {
        let b = *aml.get(at + 1 + i)? as usize;
        value |= b << (4 + 8 * i);
    }
    Some((value, 1 + follow))
}

/// One integer data object: (value, bytes used).
fn integer(aml: &[u8], at: usize) -> Option<(u64, usize)> {
    let op = *aml.get(at)?;
    let le = |n: usize| -> Option<u64> {
        let s = aml.get(at + 1..at + 1 + n)?;
        let mut v = 0u64;
        for (i, b) in s.iter().enumerate() {
            v |= (*b as u64) << (8 * i);
        }
        Some(v)
    };
    match op {
        ZERO_OP => Some((0, 1)),
        ONE_OP => Some((1, 1)),
        ONES_OP => Some((u64::MAX, 1)),
        BYTE_PREFIX => Some((le(1)?, 2)),
        WORD_PREFIX => Some((le(2)?, 3)),
        DWORD_PREFIX => Some((le(4)?, 5)),
        QWORD_PREFIX => Some((le(8)?, 9)),
        _ => None,
    }
}

/// Decode a `Package` that starts at `at` (on the PackageOp byte).
fn package(aml: &[u8], at: usize) -> Option<SleepPackage> {
    if *aml.get(at)? != PACKAGE_OP {
        return None;
    }
    let (len, used) = pkg_length(aml, at + 1)?;
    let end = (at + 1).checked_add(len)?;
    if end > aml.len() {
        return None;
    }
    let count = *aml.get(at + 1 + used)? as usize;
    let mut cursor = at + 2 + used;
    let (first, n) = integer(aml, cursor)?;
    cursor += n;
    if cursor > end {
        return None;
    }
    match count {
        0 => None,
        1 => Some(SleepPackage {
            slp_typ_a: (first as u8) & 0x7,
            slp_typ_b: ((first >> 8) as u8) & 0x7,
        }),
        _ => {
            let (second, n) = integer(aml, cursor)?;
            if cursor + n > end {
                return None;
            }
            Some(SleepPackage { slp_typ_a: (first as u8) & 0x7, slp_typ_b: (second as u8) & 0x7 })
        }
    }
}

/// Find `\_Sx` (x = `state`, 0..=5) in one AML block.
pub fn find_sleep_package(aml: &[u8], state: u8) -> Option<SleepPackage> {
    if state > 5 {
        return None;
    }
    let seg = [b'_', b'S', b'0' + state, b'_'];
    let mut i = 0usize;
    while i + 4 <= aml.len() {
        if aml[i..i + 4] != seg {
            i += 1;
            continue;
        }
        if let Some(found) = at_name(aml, i) {
            return Some(found);
        }
        i += 1;
    }
    None
}

/// `i` is the start of a matching NameSeg. Accept it only when the bytes
/// around it make it the name of a Name or Method definition.
fn at_name(aml: &[u8], i: usize) -> Option<SleepPackage> {
    let after = i + 4;
    // Name (_S5_, Package ...) or Name (\_S5_, Package ...)
    let name_start = if i >= 1 && aml[i - 1] == ROOT_CHAR { i - 1 } else { i };
    if name_start >= 1 && aml[name_start - 1] == NAME_OP {
        return package(aml, after);
    }
    // Method (_S5_, 0) { Return (Package ...) }: MethodOp PkgLength NameString
    // MethodFlags ReturnOp Package. The PkgLength is 1 to 4 bytes long and
    // sits between the opcode and the name.
    for lead in 1..=4usize {
        if name_start < lead + 1 {
            break;
        }
        let op_at = name_start - lead - 1;
        if aml[op_at] != METHOD_OP {
            continue;
        }
        match pkg_length(aml, op_at + 1) {
            Some((_, used)) if used == lead => {}
            _ => continue,
        }
        if aml.get(after + 1) == Some(&RETURN_OP) {
            return package(aml, after + 2);
        }
    }
    None
}

/// Search the DSDT first and then each SSDT, as the namespace would resolve
/// a root-level name defined in exactly one of them.
pub fn find_in_blocks<'a, I: IntoIterator<Item = &'a [u8]>>(
    blocks: I,
    state: u8,
) -> Option<SleepPackage> {
    blocks.into_iter().find_map(|b| find_sleep_package(b, state))
}
