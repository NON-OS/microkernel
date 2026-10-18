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

//! The RSDP the loader named, for the table readers that are not the boot
//! path's: they ask here rather than each reading the tree again.

use core::sync::atomic::{AtomicU64, Ordering};

/// Zero until the boot CPU reads the tree; zero is never an RSDP
/// (`Fdt::acpi_rsdp` refuses it), so it doubles as "none".
static RSDP: AtomicU64 = AtomicU64::new(0);

/// The boot CPU, once, while it reads the tree and before any other runs.
pub(super) fn record(rsdp: Option<u64>) {
    RSDP.store(rsdp.unwrap_or(0), Ordering::Release);
}

/// The RSDP's physical address, if the loader named one in `/chosen`
/// (`nonos,acpi-rsdp`).
pub fn acpi_rsdp() -> Option<u64> {
    match RSDP.load(Ordering::Acquire) {
        0 => None,
        at => Some(at),
    }
}
