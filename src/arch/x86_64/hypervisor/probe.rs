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

/* Reading the leaves. CPUID is the only thing here that touches the CPU. */

use super::leaves::{Leaves, LEAF_ALT, LEAF_BASE};
use crate::arch::x86_64::cpu::cpuid;

pub(super) fn read() -> Leaves {
    let (_, _, ecx, _) = cpuid(1);
    let present = ecx & (1 << 31) != 0;
    if !present {
        return Leaves::default();
    }
    let (max, b, c, d) = cpuid(LEAF_BASE);
    let mut l = Leaves { present, max, sig: [b, c, d], ..Leaves::default() };
    if max > LEAF_BASE {
        l.interface = cpuid(LEAF_BASE + 1).0;
    }
    // Only a hypervisor that answers the base leaf with a real range gets the
    // alternate one read; elsewhere the leaf would echo unrelated words.
    if max >= LEAF_BASE {
        let (alt_max, b, c, d) = cpuid(LEAF_ALT);
        if alt_max >= LEAF_ALT {
            l.alt = [b, c, d];
        }
    }
    l
}
