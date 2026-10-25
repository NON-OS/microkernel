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

//! Every capability a mask holds, by the kernel's name, and any bit this
//! build has no name for as `unknown bit N`: a capability the kernel added
//! after this terminal was built is shown, never dropped.

use alloc::vec::Vec;

use crate::command::builtin::cap_names::CAP_NAMES;
use crate::term::util::format_u64;

pub fn names(caps: u64) -> Vec<Vec<u8>> {
    let held = (0..64).filter(|i| caps & (1u64 << i) != 0);
    held.map(|i| match CAP_NAMES.get(i) {
        Some(n) => Vec::from(*n),
        None => {
            let mut v = Vec::from(&b"unknown bit "[..]);
            let mut num = [0u8; 4];
            let k = format_u64(i as u64, &mut num);
            v.extend_from_slice(&num[..k]);
            v
        }
    })
    .collect()
}
