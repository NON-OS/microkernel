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

//! The reservations published so far. Only ever added to: a restarted
//! capsule publishing again cannot take one away, and a range firmware
//! keeps stays kept until the machine resets.

extern crate alloc;

use alloc::vec::Vec;
use spin::RwLock;

use super::judge;
use super::wire::Reservation;

/// Four full publishes' worth; more is a capsule publishing in a loop.
const MAX_KEPT: usize = 1024;

static RESERVED: RwLock<Vec<Reservation>> = RwLock::new(Vec::new());

/// Adds the ranges not already kept, answering how many are kept in all,
/// or None (and nothing added) when they would not fit.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub fn add(ranges: &[Reservation]) -> Option<usize> {
    let mut kept = RESERVED.write();
    let new: Vec<Reservation> = ranges.iter().filter(|r| !kept.contains(r)).copied().collect();
    if kept.len() + new.len() > MAX_KEPT {
        return None;
    }
    kept.extend(new);
    Some(kept.len())
}

pub fn conflict_with(space: u8, base: u64, len: u64, own_bar: bool) -> Option<Reservation> {
    judge::conflict(&RESERVED.read(), space, base, len, own_bar)
}

#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub fn count() -> usize {
    RESERVED.read().len()
}

/// Whether the kept reservations cover the range, for the ECAM check.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub fn kept_cover(space: u8, base: u64, len: u64) -> bool {
    judge::covered(&RESERVED.read(), space, base, len)
}
