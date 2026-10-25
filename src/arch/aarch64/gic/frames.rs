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

//! Each CPU's own redistributor frame.
//!
//! SGIs and PPIs are configured in the frame of the CPU they belong to. The
//! region base is only the first frame, whoever owns it, so a secondary that
//! enabled its timer PPI there enabled someone else's. Each CPU records its
//! frame here when it brings up its half of the GIC.

use core::sync::atomic::{AtomicU64, Ordering};

use super::redistributor::GicRedistributor;
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::aarch64::cpu::cpu_id;

static FRAMES: [AtomicU64; MAX_CPUS] = [const { AtomicU64::new(0) }; MAX_CPUS];

pub(super) fn record(frame: &GicRedistributor) {
    FRAMES[cpu_id()].store(frame.base(), Ordering::Release);
}

/// The calling CPU's frame, `None` before it has brought up its GIC half.
pub(super) fn this_cpu() -> Option<GicRedistributor> {
    match FRAMES[cpu_id()].load(Ordering::Acquire) {
        0 => None,
        base => Some(GicRedistributor::new(base)),
    }
}
