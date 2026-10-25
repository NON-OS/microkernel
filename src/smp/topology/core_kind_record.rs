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

/// On `cpu` itself, since the kind is read from the core that asks.
pub(crate) fn record_core_kind(cpu: usize, apic_id: u32) {
    if cpu >= MAX_CPUS {
        return;
    }
    let Some(kind) = crate::arch::own_core_kind() else { return };
    HYBRID.store(true, Ordering::Release);
    KIND[cpu].store(kind as u8, Ordering::Release);
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] cpu=").dec(cpu as u64).str(b" apic=").dec(u64::from(apic_id));
    l.str(b" core=").str(kind.letter()).end();
}

pub(crate) fn core_kind(cpu: usize) -> CoreKind {
    KIND.get(cpu).map_or(CoreKind::Unknown, |k| CoreKind::from_u8(k.load(Ordering::Acquire)))
}

/// Whether any CPU said it is part of a hybrid part.
pub(crate) fn is_hybrid() -> bool {
    HYBRID.load(Ordering::Acquire)
}
