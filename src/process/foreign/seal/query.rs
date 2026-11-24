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

//! What the seal can say about a live family without opening it: which
//! family it is, and whether a measurement is among those the kernel
//! recorded for it. The list holds
//! only what MkLocalVerify and MkCapsuleVerify accepted for the family's
//! supervisor (absorb.rs), so a yes is the kernel's own word that it
//! verified those bytes for this family. A family past its bound answers
//! only from the measurements it kept, so an unkept one is a no.

use super::table::with_family;

/// Whether the live family `sup` supervises recorded `measurement` under one
/// of `kinds` (kind.rs).
pub(crate) fn holds(sup: u32, measurement: &[u8; 32], kinds: u8) -> bool {
    with_family(sup, |f| {
        let at = f.list.iter().position(|m| m == measurement);
        at.is_some_and(|i| f.kinds.get(i).is_some_and(|k| k & kinds != 0))
    })
    .unwrap_or(false)
}

/// The generation of the live family `sup` supervises, or 0 when it has none:
/// a family sealed and the next reserved under the same supervisor answer
/// differently, so nothing granted to one is taken for the other.
pub(crate) fn generation(sup: u32) -> u64 {
    with_family(sup, |f| f.generation).unwrap_or(0)
}
