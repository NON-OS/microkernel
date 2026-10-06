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

//! Each CPU's core kind, read on that CPU at bring-up, and the line that
//! names it on a hybrid part.

use super::core_kind::CoreKind;
use crate::smp::MAX_CPUS;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

static KIND: [AtomicU8; MAX_CPUS] = [const { AtomicU8::new(0) }; MAX_CPUS];
static HYBRID: AtomicBool = AtomicBool::new(false);

/// On `cpu` itself: leaf 1AH answers for the core that executes it.
pub(crate) fn record_core_kind(cpu: usize, apic_id: u32) {
    if cpu >= MAX_CPUS {
        return;
    }
    let Some(kind) = read_own_kind() else { return };
    HYBRID.store(true, Ordering::Release);
    KIND[cpu].store(kind as u8, Ordering::Release);
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] cpu=").dec(cpu as u64).str(b" apic=").dec(u64::from(apic_id));
    l.str(b" core=").str(kind.letter()).end();
}

/// None on a part that is not hybrid, where every core is alike.
#[cfg(target_arch = "x86_64")]
fn read_own_kind() -> Option<CoreKind> {
    use super::core_kind::{LEAF7_EDX_HYBRID, LEAF_CORE_TYPE};
    use crate::arch::x86_64::boot::cpu_ops::cpuid_count;
    let max = cpuid_count(0, 0).0;
    if max < 7 || cpuid_count(7, 0).3 & LEAF7_EDX_HYBRID == 0 {
        return None;
    }
    if max < LEAF_CORE_TYPE {
        return Some(CoreKind::Unknown);
    }
    Some(CoreKind::from_leaf_1a(cpuid_count(LEAF_CORE_TYPE, 0).0))
}

#[cfg(not(target_arch = "x86_64"))]
fn read_own_kind() -> Option<CoreKind> {
    None
}

pub(crate) fn core_kind(cpu: usize) -> CoreKind {
    KIND.get(cpu).map_or(CoreKind::Unknown, |k| CoreKind::from_u8(k.load(Ordering::Acquire)))
}

/// Whether any CPU said it is part of a hybrid part.
pub(crate) fn is_hybrid() -> bool {
    HYBRID.load(Ordering::Acquire)
}
