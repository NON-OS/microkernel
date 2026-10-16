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

//! Every table the kernel knows, read once for measurement (F-90), so the
//! list covers the tables the kernel parses itself (MADT, DMAR or IVRS,
//! MCFG and the rest) as well as the AML the capsule loads, and is all
//! measured before any `_INI` can run. A fixed order: the FADT, the DSDT,
//! each SSDT as the root table lists them, which is the capsule's load
//! order, then the rest by signature value. The FACS is left out, since
//! firmware writes it (the waking vector, the Global Lock).

use super::kernel::platform::fw::parser::state::TABLES;
use super::table_check::{declared_length, verify, TableError, HEADER};
use super::table_measure::Measured;
use super::tables::{locate, read};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

static SWEPT: AtomicBool = AtomicBool::new(false);

/// The sweep, the first time only: each table that reads whole, to `add`.
pub(super) fn sweep_once<T>(mut add: impl FnMut(Measured) -> T) {
    if SWEPT.swap(true, Ordering::AcqRel) {
        return;
    }
    for (s, i) in every_table() {
        if let Ok(b) = copy_any(s, i) {
            add(Measured::of(s, i, &b));
        }
    }
}

const OWN_PLACE: [[u8; 4]; 4] = [*b"FACP", *b"DSDT", *b"SSDT", *b"FACS"];

fn every_table() -> Vec<([u8; 4], u32)> {
    let (ssdts, rest) = {
        let tables = TABLES.read();
        let Some(reg) = tables.as_ref() else { return Vec::new() };
        let sigs = reg.tables.keys().map(|k| k.to_le_bytes());
        (
            reg.ssdt_addresses.len() as u32,
            sigs.filter(|s| !OWN_PLACE.contains(s)).collect::<Vec<_>>(),
        )
    };
    let mut out = alloc::vec![(*b"FACP", 0), (*b"DSDT", 0)];
    out.extend((0..ssdts).map(|i| (*b"SSDT", i)));
    out.extend(rest.into_iter().map(|s| (s, 0)));
    out
}

/// A table as checked as the capsule's copy, whether or not the capsule may
/// read it: the measurement stays in the kernel.
fn copy_any(sig: [u8; 4], instance: u32) -> Result<Vec<u8>, TableError> {
    let phys = locate(sig, instance).ok_or(TableError::Absent)?;
    let head = read(phys, HEADER)?;
    let bytes = read(phys, declared_length(&head, sig)?)?;
    verify(&bytes, sig)?;
    Ok(bytes)
}
