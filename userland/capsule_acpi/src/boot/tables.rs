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

//! The tables the capsule starts from, as the kernel hands them over: the
//! FADT, the DSDT and every SSDT, and the FACS mapped for the Global Lock.

use crate::sys::acpi::{map, table};
use crate::sys::log::Line;
use alloc::vec::Vec;

/// Far more SSDTs than any firmware ships.
const MAX_SSDTS: u32 = 256;
/// No such table: the instances have run out.
const ENOENT: i64 = -2;
/// The FACS is 64 bytes in every revision (ACPI 6.5, 5.2.10).
const FACS_LEN: u64 = 64;

pub struct Tables {
    pub fadt: Vec<u8>,
    /// The DSDT first, then the SSDTs in the order the root table lists them.
    pub blocks: Vec<Vec<u8>>,
}

pub fn fetch() -> Option<Tables> {
    let Ok(fadt) = table(*b"FACP", 0) else {
        Line::new(b"no FADT; nothing to run").end();
        return None;
    };
    let Ok(dsdt) = table(*b"DSDT", 0) else {
        Line::new(b"no DSDT; nothing to run").end();
        return None;
    };
    let mut blocks = alloc::vec![dsdt];
    for i in 0..MAX_SSDTS {
        match table(*b"SSDT", i) {
            Ok(t) => blocks.push(t),
            // The kernel refused this one (and said why); the next may load.
            Err(e) if e != ENOENT => continue,
            Err(_) => break,
        }
    }
    Line::new(b"tables: DSDT and ").dec(blocks.len() as u64 - 1).str(b" SSDTs").end();
    Some(Tables { fadt, blocks })
}

/// X_FIRMWARE_CTRL when the FADT has it and it is set, else FIRMWARE_CTRL
/// (5.2.9), mapped through the kernel's rule (the FACS lives in ACPI NVS).
pub fn map_facs(fadt: &[u8]) -> Option<u64> {
    let x = fadt.get(132..140).map_or(0, |b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])));
    let legacy = fadt.get(36..40).map_or(0, |b| u32::from_le_bytes(b.try_into().unwrap_or([0; 4])));
    let phys = if x != 0 { x } else { legacy as u64 };
    if phys == 0 {
        return None;
    }
    match map(phys, FACS_LEN) {
        Ok((va, _)) => Some(va),
        Err(e) => {
            Line::new(b"FACS not mapped, errno ").dec(e.unsigned_abs()).end();
            None
        }
    }
}
