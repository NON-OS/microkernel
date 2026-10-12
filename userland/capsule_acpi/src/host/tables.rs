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

//! Tables by signature and OEM ids, for DataTableRegion and LoadTable
//! (ACPI 6.5, 19.6.32 and 19.6.76): every instance the kernel will hand
//! over is looked at, and the first whose ids match is the one.

use crate::sys::acpi::table;
use alloc::vec::Vec;

/// Far more instances of one signature than any firmware ships.
const MAX_INSTANCES: u32 = 64;

/// An id as AML passes it may be shorter than the header's field, which is
/// padded; an empty one matches anything.
fn matches(field: &[u8], want: &[u8]) -> bool {
    let want = want.split(|b| *b == 0).next().unwrap_or(&[]);
    let field = field.split(|b| *b == 0).next().unwrap_or(&[]);
    let trim = |s: &[u8]| s.iter().rposition(|b| *b != b' ').map_or(0, |i| i + 1);
    want.is_empty() || field[..trim(field)] == want[..trim(want)]
}

pub fn find(sig: &[u8], oem_id: &[u8], oem_table_id: &[u8]) -> Option<Vec<u8>> {
    let sig: [u8; 4] = sig.try_into().ok()?;
    for i in 0..MAX_INSTANCES {
        let t = table(sig, i).ok()?;
        if t.len() >= 24 && matches(&t[10..16], oem_id) && matches(&t[16..24], oem_table_id) {
            return Some(t);
        }
    }
    None
}
