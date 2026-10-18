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

//! Whether a platform device the ACPI capsule describes may join the device
//! table. Pure: `kernel_proofs` mounts it beside the region rule. Its
//! windows must be what the region rule would grant (firmware's memory or
//! device space, no other capsule's window, none of the kernel's own
//! hardware), and must not overlap a device already in the table; a window
//! the table already holds exactly means the device is there already (the
//! kernel's own ACPI scan found it), and it is not added twice.

use super::platform_wire::Platform;
use super::rule::{memory, Facts, Refusal};

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    New,
    /// The table holds this window already, under this device id.
    Present(u64),
    Overlaps(u64),
    Refused(Refusal),
}

/// `table` holds each existing record's MMIO windows as (device id, base,
/// length).
pub fn admit(p: &Platform, table: &[(u64, u64, u64)], facts: &impl Facts) -> Verdict {
    for &(base, len) in &p.windows {
        if let Some(&(id, ..)) = table.iter().find(|(_, b, l)| *b == base && *l == len) {
            return Verdict::Present(id);
        }
        let end = base.saturating_add(len);
        if let Some(&(id, ..)) =
            table.iter().find(|(_, b, l)| base < b.saturating_add(*l) && *b < end)
        {
            return Verdict::Overlaps(id);
        }
        if let Err(r) = memory(base, len, facts) {
            return Verdict::Refused(r);
        }
    }
    Verdict::New
}
