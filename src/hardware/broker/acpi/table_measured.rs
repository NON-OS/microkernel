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
//! the first hand-out to the capsule (table_sweep), each table the capsule
//! is handed, which adds a record only if its bytes changed, and each table
//! AML loads at run time (table_loaded). The digest over the list replaces the
//! firmware entry in the attest registry, so the root an attestation signs
//! covers the AML the machine runs.

use super::loaded_rule::{admit, Admit};
use super::table_measure::{digest, Measured, RECORD_LEN};
use super::table_sweep::sweep_once;
use crate::security::attest_registry::{mark_firmware_cut, record_firmware};
use alloc::vec::Vec;
use spin::Mutex;

/// More than any firmware ships; past it the registry says the machine
/// can no longer describe itself.
const MAX_TABLES: usize = 128;

static LIST: Mutex<Vec<Measured>> = Mutex::new(Vec::new());

pub fn record(sig: [u8; 4], instance: u32, bytes: &[u8]) {
    sweep_once(add);
    add(Measured::of(sig, instance, bytes));
}

pub(super) fn add(m: Measured) -> Admit {
    let mut list = LIST.lock();
    let a = admit(&mut list, m, MAX_TABLES);
    let all = digest(&list);
    drop(list);
    match a {
        Admit::Full => mark_firmware_cut(),
        Admit::Added => {
            record_firmware(all);
            super::table_report::measured(&m);
        }
        Admit::Known => {}
    }
    a
}

/// The records in order, RECORD_LEN bytes each.
pub fn records() -> Vec<u8> {
    let list = LIST.lock();
    let mut out = Vec::with_capacity(list.len() * RECORD_LEN);
    list.iter().for_each(|m| out.extend_from_slice(&m.encode()));
    out
}
