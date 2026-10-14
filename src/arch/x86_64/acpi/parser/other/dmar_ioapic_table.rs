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

//! The IO-APIC scopes of segment 0's DRHDs, kept from boot for the
//! remapping entries the IO-APIC's lines need (K3).

use spin::Mutex;

use super::dmar_ioapic::{ioapic_scopes, IoapicScope, MAX_IOAPIC_SCOPES};
use super::dmar_ioapic_source::source_id;
use super::dmar_scope::DRHD_HEADER_LEN;

static SCOPES: Mutex<[Option<IoapicScope>; MAX_IOAPIC_SCOPES]> =
    Mutex::new([None; MAX_IOAPIC_SCOPES]);

pub(super) fn record(drhd: &[u8]) {
    ioapic_scopes(drhd, DRHD_HEADER_LEN, &mut SCOPES.lock()[..]);
}

/// The requester id of the IO-APIC whose MADT id is `id`, each bridge on
/// its path read for its secondary bus by `secondary`; None when no DRHD
/// names it.
pub fn ioapic_source(id: u8, secondary: impl Fn(u8, u8, u8) -> Option<u8>) -> Option<u16> {
    let scope = SCOPES.lock().iter().flatten().find(|s| s.id == id).copied()?;
    source_id(&scope, secondary)
}
