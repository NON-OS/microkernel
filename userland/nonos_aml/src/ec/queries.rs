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

//! The EC's query methods (ACPI 6.5, 12.6.2): _Qxx under the EC device runs
//! for query value xx, in hexadecimal.

use crate::namespace::{Id, Kind, Namespace};
use alloc::collections::BTreeMap;

fn hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub fn query_methods(ns: &Namespace, ec: Id) -> BTreeMap<u8, Id> {
    let mut out = BTreeMap::new();
    for id in ns.children(ec) {
        let Some(e) = ns.get(id) else { continue };
        if !matches!(e.kind, Kind::Method(_)) || e.seg[..2] != *b"_Q" {
            continue;
        }
        if let (Some(hi), Some(lo)) = (hex(e.seg[2]), hex(e.seg[3])) {
            out.insert(hi * 16 + lo, id);
        }
    }
    out
}
