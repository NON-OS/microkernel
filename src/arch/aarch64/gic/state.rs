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

use core::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use super::version::GicVersion;

/// Written once by the boot CPU before any secondary or interrupt exists.
static VERSION: AtomicU8 = AtomicU8::new(0);
static DIST_BASE: AtomicU64 = AtomicU64::new(0);
/// GICv2's CPU interface; unused on GICv3.
static CPU_BASE: AtomicU64 = AtomicU64::new(0);
/// GICv3's redistributor region; unused on GICv2.
static REDIST_BASE: AtomicU64 = AtomicU64::new(0);

pub(super) fn set_layout(version: GicVersion, dist: u64, cpu: u64, redist: u64) {
    DIST_BASE.store(dist, Ordering::Release);
    CPU_BASE.store(cpu, Ordering::Release);
    REDIST_BASE.store(redist, Ordering::Release);
    let tag = match version {
        GicVersion::V2 => 2,
        GicVersion::V3 => 3,
    };
    VERSION.store(tag, Ordering::Release);
}

/// `None` until `init_gic` has chosen, which keeps every operation before it
/// a no-op instead of a guess at registers.
pub(super) fn version() -> Option<GicVersion> {
    match VERSION.load(Ordering::Acquire) {
        2 => Some(GicVersion::V2),
        3 => Some(GicVersion::V3),
        _ => None,
    }
}

pub(super) fn dist_base() -> u64 {
    DIST_BASE.load(Ordering::Acquire)
}

pub(super) fn cpu_base() -> u64 {
    CPU_BASE.load(Ordering::Acquire)
}

pub(super) fn redist_base() -> u64 {
    REDIST_BASE.load(Ordering::Acquire)
}
