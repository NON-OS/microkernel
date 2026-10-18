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

//! The table's rules, checked by `nonos_smbios_proofs` over the table that
//! ships: every entry cites a reason, names at least one field with a
//! pattern, names its device, and has a name of its own.

use super::types::{DeviceMatch, Match, Quirk};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Problem {
    NoReason,
    NoFields,
    EmptyPattern,
    NoDevice,
    DuplicateName,
}

pub fn check(table: &[Quirk]) -> Result<(), (&'static str, Problem)> {
    for (i, q) in table.iter().enumerate() {
        let fail = |p| Err((q.name, p));
        if q.reason.trim().is_empty() {
            return fail(Problem::NoReason);
        }
        if q.dmi.is_empty() {
            return fail(Problem::NoFields);
        }
        let empty = |m: &Match| matches!(m, Match::Exact(p) | Match::Prefix(p) if p.is_empty());
        if q.dmi.iter().any(|m| empty(&m.is)) {
            return fail(Problem::EmptyPattern);
        }
        if matches!(q.device, DeviceMatch::Acpi { hid } if hid.is_empty()) {
            return fail(Problem::NoDevice);
        }
        if table[..i].iter().any(|p| p.name == q.name) {
            return fail(Problem::DuplicateName);
        }
    }
    Ok(())
}
