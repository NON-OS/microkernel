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

//! Each CPU's GICv2 CPU interface number, as the one-hot mask SGIs and SPI
//! targets are written with.
//!
//! GICv2 does not address CPUs by affinity. Each CPU learns its own mask by
//! reading the banked `GICD_ITARGETSR0`, so it is recorded here, by dense CPU
//! index, when that CPU brings up its half of the GIC.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::aarch64::cpu::cpu_id;

static MASKS: [AtomicU8; MAX_CPUS] = [const { AtomicU8::new(0) }; MAX_CPUS];

pub(super) fn record(mask: u8) {
    MASKS[cpu_id()].store(mask, Ordering::Release);
}

/// The mask of the CPU whose affinity is `packed` (`Aff3:Aff2:Aff1:Aff0`, a
/// byte each), or `None` before that CPU is up or if no CPU has it.
pub(in crate::arch::aarch64::gic) fn of_affinity(packed: u32) -> Option<u8> {
    let p = packed as u64;
    let mpidr = ((p >> 24) << 32) | (p & 0x00FF_FFFF);
    let index = roster::index_of(mpidr)?;
    match MASKS.get(index)?.load(Ordering::Acquire) {
        0 => None,
        mask => Some(mask),
    }
}
