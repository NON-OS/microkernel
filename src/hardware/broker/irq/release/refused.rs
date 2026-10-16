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

//! What a bind refused at its last step undoes (L69, L73). Its claim went
//! while it ran, so a newer claim may already hold the device: what the bind
//! wrote to the device is taken back only while nothing holds it, so a newer
//! claimant's vectors are never cleared, and what it took from the kernel's
//! books is always given back.

use super::super::grant::{IrqGrant, IrqGrantKind};
use super::super::{bind as bind_internal, slots};
use crate::arch::interrupt::broker::slot_of;
use crate::arch::interrupt::ioapic;
use crate::hardware::broker::claim;

pub(in super::super) fn undo_refused(grants: &[IrqGrant]) {
    // One claim per bind, so one device.
    if let Some(g) = grants.first() {
        let _ = claim::while_unclaimed(g.device_id, || grants.iter().for_each(device_half));
    }
    grants.iter().for_each(books_half);
}

// None of these grants was entered, and nothing holds the device: MSI-X
// goes off whole, without asking the records what else is bound.
fn device_half(g: &IrqGrant) {
    match g.kind {
        IrqGrantKind::Intx => {}
        IrqGrantKind::Msix => {
            bind_internal::teardown_msix_vector(g.device_id, g.device_vector);
            bind_internal::disable_msix_for_device(g.device_id);
        }
        IrqGrantKind::Msi => bind_internal::disable_msi_for_device(g.device_id),
    }
}

// The line, the IRTE and the broker vector the bind took. The line was the
// bind's own until now, so no newer claimant can have bound it.
fn books_half(g: &IrqGrant) {
    match g.kind {
        IrqGrantKind::Intx => {
            let _ = ioapic::mask(g.irq_source, true);
            let _ = ioapic::release_gsi_from_capsule(g.irq_source);
        }
        IrqGrantKind::Msix | IrqGrantKind::Msi => bind_internal::release_irte(g.irte),
    }
    if let Some(idx) = slot_of(g.vector) {
        slots::deactivate(idx);
        slots::free_slot(idx);
    }
}
