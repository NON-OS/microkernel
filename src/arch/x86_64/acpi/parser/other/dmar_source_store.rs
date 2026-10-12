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

//! The interrupt sources DMAR scopes to a remapping unit: each I/O APIC and
//! MSI-capable HPET with the requester id its interrupt messages carry. An
//! entry remapping one of their lines must name that id, so a strict
//! remapping (L44) is possible only for a source listed here.

use core::sync::atomic::{AtomicUsize, Ordering};

use spin::Mutex;

use super::dmar_namespace_scope::{NamespaceScope, SCOPE_HPET, SCOPE_IOAPIC};
use super::dmar_namespace_walk::walk_numbered_scopes;

const MAX_SOURCES: usize = 16;
/// A DRHD's device scopes start after its 16-byte header (VT-d 8.3).
const DRHD_SCOPES_AT: usize = 16;

/// (index of the unit in `remap_unit_bases`, the scope it lists).
type Scoped = heapless::Vec<(u8, NamespaceScope), MAX_SOURCES>;

static IOAPICS: Mutex<Scoped> = Mutex::new(heapless::Vec::new());
static HPETS: Mutex<Scoped> = Mutex::new(heapless::Vec::new());
static DROPPED: AtomicUsize = AtomicUsize::new(0);

/// `drhd` is the whole structure of the unit at `unit` in table order.
pub(super) fn record_source_scopes(unit: u8, drhd: &[u8]) {
    for (kind, store) in [(SCOPE_IOAPIC, &IOAPICS), (SCOPE_HPET, &HPETS)] {
        let mut found = [NamespaceScope::EMPTY; MAX_SOURCES];
        let (count, dropped) = walk_numbered_scopes(drhd, DRHD_SCOPES_AT, kind, &mut found);
        let mut lost = usize::from(dropped);
        let mut kept = store.lock();
        for scope in found.iter().take(count as usize) {
            lost += usize::from(kept.push((unit, *scope)).is_err());
        }
        DROPPED.fetch_add(lost, Ordering::AcqRel);
    }
}

fn find(store: &Mutex<Scoped>, number: u8) -> Option<(u8, (u8, u8, u8))> {
    let kept = store.lock();
    let (unit, scope) = kept.iter().find(|(_, s)| s.number == number)?;
    Some((*unit, scope.requester()?))
}

/// The unit and requester id of the I/O APIC with APIC id `id`, when DMAR
/// scopes it with a path the kernel can resolve.
pub fn ioapic_source(id: u8) -> Option<(u8, (u8, u8, u8))> {
    find(&IOAPICS, id)
}

/// The same for HPET block `number`.
pub fn hpet_source(number: u8) -> Option<(u8, (u8, u8, u8))> {
    find(&HPETS, number)
}

/// Scopes of either kind DMAR listed that were not kept.
pub fn source_scopes_dropped() -> usize {
    DROPPED.load(Ordering::Acquire)
}
