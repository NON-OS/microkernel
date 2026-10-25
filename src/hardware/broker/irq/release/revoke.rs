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

//! A depart's revocation of a device's IRQ grants (`acpi::depart`), bound to
//! the claim it retired as a release's drain is (L71).

use super::super::records::drain_for_device;
use super::unwind::teardown;

/// A depart's revoke: as `release_for_device`, and each grant remembered
/// as departed so a later poll or wait on it answers ENODEV.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub fn revoke_for_device(pid: u32, device_id: u64, epoch: u64) -> usize {
    let drained = drain_for_device(pid, device_id, epoch);
    for g in &drained {
        teardown(g);
        super::super::gone::mark(pid, g.grant_id);
    }
    drained.len()
}
