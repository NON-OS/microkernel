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

//! The measured tables, kept: every table the kernel knows, swept once at
//! the first hand-out to the capsule (table_sweep), then each table the
//! capsule is handed, which adds a record only if its bytes changed. The
//! digest over the list replaces the firmware entry in the attest registry,
//! so the root an attestation signs covers the AML the machine runs.
//! Read-only: nothing is refused on it.

use super::table_measure::{digest, note, Measured, RECORD_LEN};
use super::table_sweep::{copy_any, every_table};
use crate::security::attest_registry::{mark_firmware_cut, record_firmware};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

/// More than any firmware ships; past it the registry says the machine
/// can no longer describe itself.
const MAX_TABLES: usize = 128;

static LIST: Mutex<Vec<Measured>> = Mutex::new(Vec::new());
static SWEPT: AtomicBool = AtomicBool::new(false);

pub fn record(sig: [u8; 4], instance: u32, bytes: &[u8]) {
    if !SWEPT.swap(true, Ordering::AcqRel) {
        for (s, i) in every_table() {
            if let Ok(b) = copy_any(s, i) {
                add(Measured::of(s, i, &b));
            }
        }
    }
    add(Measured::of(sig, instance, bytes));
}

fn add(m: Measured) {
    let mut list = LIST.lock();
    if list.len() >= MAX_TABLES && !list.contains(&m) {
        return mark_firmware_cut();
    }
    if !note(&mut list, m) {
        return;
    }
    let all = digest(&list);
    drop(list);
    record_firmware(all);
    super::table_report::measured(&m);
}

/// The records in order, RECORD_LEN bytes each.
pub fn records() -> Vec<u8> {
    let list = LIST.lock();
    let mut out = Vec::with_capacity(list.len() * RECORD_LEN);
    list.iter().for_each(|m| out.extend_from_slice(&m.encode()));
    out
}
