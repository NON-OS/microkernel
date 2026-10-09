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

//! The AMD IOMMUs this kernel drives, in IVRS order, and whether they are
//! translating with its tables.

use core::sync::atomic::{AtomicBool, Ordering};

use spin::Once;

use super::mmio::Unit;
use crate::arch::x86_64::acpi::parser::other::ivrs_walk::MAX_AMD_IOMMUS;

pub(super) type Units = heapless::Vec<Unit, MAX_AMD_IOMMUS>;

static UNITS: Once<Units> = Once::new();
static ENFORCING: AtomicBool = AtomicBool::new(false);

pub(super) fn record(units: Units) -> &'static [Unit] {
    UNITS.call_once(|| units).as_slice()
}

/// Every unit mapped at bring-up, empty before it.
pub fn units() -> &'static [Unit] {
    UNITS.get().map(|u| u.as_slice()).unwrap_or(&[])
}

/// True once every unit acknowledged IOMMU_EN with the kernel's device table.
pub fn is_enforcing() -> bool {
    ENFORCING.load(Ordering::Acquire)
}

pub(super) fn set_enforcing() {
    ENFORCING.store(true, Ordering::Release);
}
