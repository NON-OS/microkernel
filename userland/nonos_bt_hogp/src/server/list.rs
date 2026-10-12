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
//! The attribute database this host serves to its peripheral. GAP asks
//! The list both Read By Type and Read By Group Type answer with: a
//! length octet, then entries of that length (Vol 3, Part F, 3.4.4.2 and
//! 3.4.4.10).

use super::find::found;
use super::serve::{Answer, ATT_MTU};

/// A value cut to what an entry with `header` octets of fields can hold.
pub fn cut(value: &[u8], header: usize) -> &[u8] {
    &value[..value.len().min(ATT_MTU - header)]
}

/// Entries as long as the first, as many as fit in ATT_MTU; Attribute Not
/// Found on the start if there are none.
pub fn list<'a>(
    op: u8,
    entries: impl Iterator<Item = (u16, Option<u16>, &'a [u8])>,
    start: u16,
) -> Answer {
    let mut out = alloc::vec![op, 0];
    for (handle, group_end, value) in entries {
        let len = 2 + group_end.map_or(0, |_| 2) + value.len();
        if (out.len() > 2 && len != usize::from(out[1])) || out.len() + len > ATT_MTU {
            break;
        }
        out[1] = len as u8;
        out.extend_from_slice(&handle.to_le_bytes());
        out.extend(group_end.iter().flat_map(|e| e.to_le_bytes()));
        out.extend_from_slice(value);
    }
    found(out, 2, start)
}
