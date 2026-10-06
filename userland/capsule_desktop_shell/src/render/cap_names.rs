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

//! Display names for capability bits, hand-synced with the kernel's
//! `capabilities/types/{defs,as_str}.rs` (bit i == 1 << i, names verbatim). A
//! set bit this table does not name is still spelled out, as `bit<N>`: a
//! consent screen must never hide a capability that was granted.

use alloc::vec::Vec;

use super::cap_table::NAMES;
use crate::server::handlers::pkg_install::push_i32;

pub(super) fn append(caps: u64, out: &mut Vec<u8>) {
    let start = out.len();
    for i in 0..64 {
        if caps & (1u64 << i) == 0 {
            continue;
        }
        if out.len() != start {
            out.extend_from_slice(b", ");
        }
        match NAMES.get(i) {
            Some(name) => out.extend_from_slice(name),
            None => {
                out.extend_from_slice(b"bit");
                push_i32(out, i as i32);
            }
        }
    }
    if out.len() == start {
        out.extend_from_slice(b"(none)");
    }
}
