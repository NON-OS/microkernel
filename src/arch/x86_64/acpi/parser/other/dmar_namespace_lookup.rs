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

//! What the kernel asks of the ACPI namespace devices DMAR declared.

use core::sync::atomic::Ordering;

use super::acpi_path::same_path;
use super::dmar_andd::Andd;
use super::dmar_namespace_store::{DECLARED, DROPPED, MAX_NAMESPACE, SCOPED};

/// Every ANDD that parsed, in table order.
pub fn namespace_devices() -> heapless::Vec<Andd, MAX_NAMESPACE> {
    DECLARED.lock().clone()
}

/// Declarations and scopes left out: malformed, or past the room kept. A
/// device among them is never confined, and stays counted as reachable.
pub fn namespace_dropped() -> usize {
    DROPPED.load(Ordering::Acquire)
}

/// The unit index and requester id DMA from the ACPI device at `path`
/// arrives under, when DMAR declares it and a unit's scope gives one hop.
pub fn namespace_requester(path: &[u8]) -> Option<(u8, (u8, u8, u8))> {
    let number = DECLARED.lock().iter().find(|a| same_path(a.name(), path))?.number;
    let scoped = SCOPED.lock();
    let (unit, scope) = scoped.iter().find(|(_, s)| s.number == number)?;
    Some((*unit, scope.requester()?))
}
