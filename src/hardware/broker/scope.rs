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

//! Which devices a driver may claim (plan/50 P6, K6f). Each driver's row in
//! scope_table.rs, keyed by the service name the kernel registers for it at
//! spawn, names what it may claim: PCI functions by vendor and device id
//! range, or by class, subclass and prog-if; ACPI records by their broker
//! class. A capsule with no row claims nothing, and a claim outside its row
//! is refused before the device is touched. The table is measured with the
//! kernel; scripts/check_driver_scope.py keeps each capsule's discover table
//! inside its row. Pure: kernel_proofs mounts this file and its tables.

// The tables sit beside this file and are mounted by path, so kernel_proofs,
// which mounts this file alone, reads the same tables.
#[path = "scope_ids.rs"]
mod ids;
#[path = "scope_lpss.rs"]
mod lpss;
#[path = "scope_rows.rs"]
mod rows;
#[path = "scope_match.rs"]
mod rule;
#[path = "scope_table.rs"]
mod table;

pub(crate) use rule::{Match, Seen};

/// The row for `service`, or None: then it claims nothing.
pub(crate) fn row(service: &str) -> Option<&'static [Match]> {
    table::SCOPE.iter().find(|(name, _)| *name == service).map(|(_, row)| *row)
}

/// Whether `service`'s row admits the device `d`.
pub(crate) fn admits(service: &str, d: &Seen) -> bool {
    row(service).is_some_and(|row| row.iter().any(|m| rule::matches(m, d)))
}
