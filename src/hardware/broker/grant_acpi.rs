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

//! The one grant no claim stands behind: an ACPI SystemMemory window the
//! ACPI capsule maps (`acpi::memory_record`). Its device id names no device,
//! so no release takes it; MkMmioUnmap and the capsule's exit find it like
//! any other. Every grant against a device goes through `insert_live`.

use super::{MmioGrant, GRANTS};

#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub(in crate::hardware::broker) fn insert_unclaimed(record: MmioGrant) {
    GRANTS.lock().push(record);
}
