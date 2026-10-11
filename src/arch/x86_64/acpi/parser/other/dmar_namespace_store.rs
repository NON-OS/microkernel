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

//! The ACPI namespace devices DMAR declares, and the remapping unit whose
//! scope names each, kept from parsing so the broker can confine such a
//! device by the requester id its DMA arrives under.

use core::sync::atomic::{AtomicUsize, Ordering};

use spin::Mutex;

pub(super) use super::dmar_andd::ANDD_TYPE;
use super::dmar_andd::{parse_andd, Andd};
use super::dmar_namespace_scope::NamespaceScope;
use super::dmar_namespace_walk::walk_namespace_scopes;

pub(super) const MAX_NAMESPACE: usize = 16;
/// A DRHD's device scopes start after its 16-byte header (VT-d 8.3).
const DRHD_SCOPES_AT: usize = 16;

pub(super) static DECLARED: Mutex<heapless::Vec<Andd, MAX_NAMESPACE>> =
    Mutex::new(heapless::Vec::new());
/// (index of the unit in `remap_unit_bases`, the scope it lists).
pub(super) static SCOPED: Mutex<heapless::Vec<(u8, NamespaceScope), MAX_NAMESPACE>> =
    Mutex::new(heapless::Vec::new());
pub(super) static DROPPED: AtomicUsize = AtomicUsize::new(0);

pub(super) fn record_andd(bytes: &[u8]) {
    if !parse_andd(bytes).is_some_and(|a| DECLARED.lock().push(a).is_ok()) {
        DROPPED.fetch_add(1, Ordering::AcqRel);
    }
}

/// `drhd` is the whole structure of the unit at `unit` in table order.
pub(super) fn record_namespace_scopes(unit: u8, drhd: &[u8]) {
    let mut found = [NamespaceScope::EMPTY; MAX_NAMESPACE];
    let (count, dropped) = walk_namespace_scopes(drhd, DRHD_SCOPES_AT, &mut found);
    let mut lost = usize::from(dropped);
    let mut scoped = SCOPED.lock();
    for scope in found.iter().take(count as usize) {
        lost += usize::from(scoped.push((unit, *scope)).is_err());
    }
    if lost > 0 {
        DROPPED.fetch_add(lost, Ordering::AcqRel);
    }
}
