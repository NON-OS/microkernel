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

//! Every processor's tables as the ACPI capsule last handed them over, and
//! its thermal limit, for the kernel's frequency and idle control (track
//! A's) to read. Nothing here acts on them: only code that owns the
//! processor enters a P-state or an idle state.

#[path = "thermal_limit.rs"]
pub mod thermal_limit;

use super::cpu_record::CpuTables;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use spin::Mutex;

/// More processors than any machine NONOS boots on; one past it is refused.
pub const MAX_CPUS: usize = 1024;

static CPUS: Mutex<Vec<CpuTables>> = Mutex::new(Vec::new());
static GENERATION: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adopted {
    New,
    Changed,
    Same,
    /// MAX_CPUS reached, or the heap could not spare the slot.
    Full,
}

pub fn adopt(t: CpuTables) -> Adopted {
    let mut cpus = CPUS.lock();
    let verdict = match cpus.iter().position(|c| c.uid == t.uid) {
        Some(i) if cpus[i] == t => return Adopted::Same,
        Some(i) => {
            cpus[i] = t;
            Adopted::Changed
        }
        None if cpus.len() >= MAX_CPUS || cpus.try_reserve(1).is_err() => return Adopted::Full,
        None => {
            cpus.push(t);
            Adopted::New
        }
    };
    GENERATION.fetch_add(1, Ordering::Release);
    verdict
}

/// The tables of the processor the MADT names by this ACPI processor UID.
pub fn tables(uid: u64) -> Option<CpuTables> {
    CPUS.lock().iter().find(|c| c.uid == uid).cloned()
}

pub fn uids() -> Vec<u64> {
    CPUS.lock().iter().map(|c| c.uid).collect()
}

/// Bumped on every change, so a reader sees _PPC or a limit move cheaply.
pub fn generation() -> u64 {
    GENERATION.load(Ordering::Acquire)
}
