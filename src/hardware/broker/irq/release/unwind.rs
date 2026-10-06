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

//! Undoing what a bind programmed, once its grant record is gone.

use super::super::grant::{IrqGrant, IrqGrantKind};
use super::super::records::count_msix_for_device;
use super::super::{bind as bind_internal, slots};
use crate::arch::interrupt::broker::slot_of;
use crate::arch::interrupt::ioapic;

// The broker vector pool stays reserved in the IO-APIC's
// `VEC_ALLOC` for the life of the kernel; the broker's own slot
// bitmap is the source of truth for which broker vectors are in
// use. Releasing the vector to `VEC_ALLOC` here would surface it
// to non-broker callers and break the reservation invariant.
pub(super) fn teardown(g: &IrqGrant) {
    match g.kind {
        IrqGrantKind::Intx => teardown_intx(g),
        IrqGrantKind::Msix => teardown_msix(g),
        IrqGrantKind::Msi => teardown_msi(g),
    }
}

fn teardown_intx(g: &IrqGrant) {
    let _ = ioapic::mask(g.irq_source, true);
    // Flip the GSI owner CAS Capsule -> Free so the next bind can
    // route the same line. The release is owner-checked; if the
    // bind path failed mid-claim and the bit was never set, the
    // release returns `GsiNotOwnedByCapsule` and we drop the error.
    let _ = ioapic::release_gsi_from_capsule(g.irq_source);
    if let Some(idx) = slot_of(g.vector) {
        slots::deactivate(idx);
        slots::free_slot(idx);
    }
}

fn teardown_msix(g: &IrqGrant) {
    bind_internal::teardown_msix_vector(g.device_id, g.device_vector);
    bind_internal::release_irte(g.irte);
    if let Some(idx) = slot_of(g.vector) {
        slots::deactivate(idx);
        slots::free_slot(idx);
    }
    if count_msix_for_device(g.device_id) == 0 {
        bind_internal::disable_msix_for_device(g.device_id);
    }
}

fn teardown_msi(g: &IrqGrant) {
    bind_internal::disable_msi_for_device(g.device_id);
    bind_internal::release_irte(g.irte);
    if let Some(idx) = slot_of(g.vector) {
        slots::deactivate(idx);
        slots::free_slot(idx);
    }
}
