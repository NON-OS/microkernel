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

//! The machine itself, told to a shipped tier and to nothing else.
//!
//! Every other family is told the declared surface (`declared`): one CPU
//! and its own address space for memory, so no two can tell they share a
//! host. A tier is the machine's own model run, sized to it: it is told
//! how many CPUs are online and how much memory there is and is free, as
//! the kernel's process table reports them, so its threads and its cache
//! fit the machine it is on.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::declared;
use super::machine_header::header;

/// The most CPUs a guest's affinity mask is written for (Linux's default
/// NR_CPUS on x86_64 is larger; musl asks about 1024).
pub const MOST_CPUS: u64 = 1024;

static SHOWN: AtomicBool = AtomicBool::new(false);
static CPUS: AtomicU64 = AtomicU64::new(0);

/// This family is a tier: from now on it is told the machine.
pub fn show() {
    SHOWN.store(true, Ordering::SeqCst);
}

/// CPUs a guest is told are online: the machine's for a tier, else one.
pub fn cpus() -> u64 {
    if !SHOWN.load(Ordering::SeqCst) {
        return declared::CPUS;
    }
    let known = CPUS.load(Ordering::Relaxed);
    if known != 0 {
        return known;
    }
    /*
     * The count does not change once the machine is up, so it is asked once.
     */
    let n = header().map_or(0, |h| u64::from(h.cpus_online)).clamp(1, MOST_CPUS);
    CPUS.store(n, Ordering::Relaxed);
    n
}

/// `(total, free)` bytes of the machine's memory for a tier, else None.
pub fn memory() -> Option<(u64, u64)> {
    if !SHOWN.load(Ordering::SeqCst) {
        return None;
    }
    let h = header()?;
    Some((h.mem_total_kb * 1024, h.mem_free_kb.min(h.mem_total_kb) * 1024))
}
