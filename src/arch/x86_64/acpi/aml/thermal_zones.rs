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

//! How many thermal zones the namespace declares, for the boot census: each
//! `ThermalZone (...)` object (ExtOpPrefix 0x5B, ThermalZoneOp 0x85), its
//! package length inside the block and its name inside the package. A byte
//! scan, as `scan` is: no AML runs here, and a pattern that does not hold
//! together is not counted.

use super::scan::{read_pkg_length, skip_name_string};

const EXT_OP_PREFIX: u8 = 0x5B;
const THERMAL_ZONE_OP: u8 = 0x85;
/// More zones than any machine declares: the count stops here.
const MAX: u16 = 64;

/// Zones in one AML block.
pub fn count(aml: &[u8]) -> u16 {
    let mut n = 0u16;
    let mut i = 0usize;
    while i + 1 < aml.len() && n < MAX {
        if aml[i] == EXT_OP_PREFIX && aml[i + 1] == THERMAL_ZONE_OP {
            let at = i + 2;
            let whole = read_pkg_length(aml, at).and_then(|(len, len_bytes)| {
                let end = at.checked_add(len).filter(|e| *e <= aml.len())?;
                skip_name_string(aml, at + len_bytes).filter(|name_end| *name_end <= end)?;
                Some(end)
            });
            if let Some(end) = whole {
                n += 1;
                i = end.max(i + 2);
                continue;
            }
        }
        i += 1;
    }
    n
}

/// Zones over every block (DSDT and SSDTs).
pub fn count_blocks<'a, I: IntoIterator<Item = &'a [u8]>>(blocks: I) -> u16 {
    blocks.into_iter().fold(0u16, |n, b| n.saturating_add(count(b)).min(MAX))
}
