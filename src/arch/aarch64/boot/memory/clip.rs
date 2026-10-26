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

//! Free RAM held to what the kernel can reach: inside a RAM region of the
//! device tree, which is what the direct map covers, and below the first
//! address the direct map could not reach. A frame outside either faults the
//! first time it is zeroed. Pure, so it is held on the host
//! (boot_memory_proofs).

use super::usable::Usable;

const PAGE: u64 = 0x1000;

/// The parts of `free` inside one of `ram` (as `[start, end)`) and below
/// `top`, page aligned inward.
pub(crate) fn within(
    free: &Usable,
    ram: impl Iterator<Item = (u64, u64)> + Clone,
    top: u64,
) -> Usable {
    let mut out = Usable::EMPTY;
    for &(a, b) in free.runs() {
        for (start, end) in ram.clone() {
            let Some(lo) = a.max(start).checked_add(PAGE - 1) else { continue };
            let (lo, hi) = (lo & !(PAGE - 1), b.min(end).min(top) & !(PAGE - 1));
            if lo < hi {
                out.add(lo, hi);
            }
        }
    }
    out
}
