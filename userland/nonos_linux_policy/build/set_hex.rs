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

//! A syscall set written as 130 hex digits, byte 0 first, as nonos.prove's
//! `[PROVE] syscall buckets ok=` line prints it.

use crate::syscall_set::{SyscallSet, SET_LEN};

pub fn set_of(hex: &str) -> Option<SyscallSet> {
    if hex.len() != SET_LEN * 2 || !hex.is_ascii() {
        return None;
    }
    let mut b = [0u8; SET_LEN];
    for (i, out) in b.iter_mut().enumerate() {
        *out = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()?;
    }
    SyscallSet::from_bytes(b).ok()
}
