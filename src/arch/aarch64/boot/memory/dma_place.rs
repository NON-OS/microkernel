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

//! Where the below-4 GiB DMA pool goes. Pure, so the rule is proved on the
//! host against real board layouts (boot_memory_proofs).
//!
//! The rule that bit: RAM need not start at zero. QEMU's virt board puts it
//! at 1 GiB with the kernel image a few hundred KiB in, so "the bottom of
//! RAM" is the image, and a pool placed there hands kernel text and the boot
//! page tables to a capsule as DMA buffers. Placement therefore never trusts
//! an address; it checks every candidate against what is in use.

const PAGE: u64 = 0x1000;
/// The top a 32-bit DMA address can name.
pub(super) const CEILING: u64 = 1 << 32;

/// The lowest page-aligned run of `want` bytes, at or above `floor` and below
/// 4 GiB, lying inside one of `available` and meeting none of `avoid`, all
/// `[start, end)`. `None` when there is no such run.
///
/// A lowest run starts either where its region starts or just past something
/// it must avoid, so those are the only places tried.
pub(super) fn place(
    available: &[(u64, u64)],
    avoid: &[(u64, u64)],
    floor: u64,
    want: u64,
) -> Option<u64> {
    if want == 0 {
        return None;
    }
    let mut lowest: Option<u64> = None;
    for &(start, end) in available {
        let top = end.min(CEILING);
        let firsts = core::iter::once(start).chain(avoid.iter().map(|&(_, after)| after));
        for first in firsts {
            let Some(at) = first.max(start).max(floor).checked_add(PAGE - 1) else {
                continue;
            };
            let at = at & !(PAGE - 1);
            let Some(stop) = at.checked_add(want) else {
                continue;
            };
            if stop > top || avoid.iter().any(|&(a, b)| a < stop && at < b) {
                continue;
            }
            lowest = Some(lowest.map_or(at, |l| l.min(at)));
        }
    }
    lowest
}
