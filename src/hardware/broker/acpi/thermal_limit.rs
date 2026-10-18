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

//! The thermal limits the ACPI capsule set, beside the processors' tables
//! (`cpu_store`), and moving the same change counter, so track A's
//! frequency control sees a new ceiling as it sees a new _PPC. Nothing in
//! the kernel acts on them until that control reads them.

#[path = "thermal_limit_rule.rs"]
pub mod rule;

use super::{GENERATION, MAX_CPUS};
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use rule::{place, Placed};
use spin::Mutex;

static LIMITS: Mutex<Vec<(u64, u8)>> = Mutex::new(Vec::new());

pub fn set(uid: u64, pct: u64) -> Placed {
    let placed = place(&mut LIMITS.lock(), uid, pct, MAX_CPUS);
    if matches!(placed, Placed::Set | Placed::Cleared) {
        GENERATION.fetch_add(1, Ordering::Release);
    }
    placed
}

/// The processor's ceiling, in percent of full performance: 100 unless
/// passive cooling has lowered it.
#[allow(dead_code)] // track A's frequency control reads it when it resumes
pub fn ceiling(uid: u64) -> u8 {
    rule::ceiling(&LIMITS.lock(), uid)
}
