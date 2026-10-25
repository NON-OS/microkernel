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

//! Which banks the kernel's events carry, from the banks the firmware's log
//! declares: every one of them, in a fixed order, if this kernel can hash
//! each at its declared size and SHA-256 is among them.

use alloc::vec::Vec;

use super::algs::{digest, ALG_SHA1, ALG_SHA256, ALG_SHA384, ALG_SHA512};

const KNOWN: [u16; 4] = [ALG_SHA1, ALG_SHA256, ALG_SHA384, ALG_SHA512];

/// The banks among `KNOWN` the header declares, if they are all it declares
/// and each has its algorithm's size. `n` is how many the header declares.
pub(crate) fn from_declared(n: usize, size_of: impl Fn(u16) -> Option<usize>) -> Option<Vec<u16>> {
    let found: Vec<u16> = KNOWN.iter().copied().filter(|a| size_of(*a).is_some()).collect();
    let sized = found.iter().all(|a| size_of(*a) == digest(*a, &[]).map(|d| d.len()));
    match found.len() == n && sized && found.contains(&ALG_SHA256) {
        true => Some(found),
        false => None,
    }
}
