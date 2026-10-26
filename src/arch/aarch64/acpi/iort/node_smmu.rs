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

//! The SMMUv1/v2 node (type 3) and the reserved memory range node (type
//! 6), Arm DEN0049 as recalled, unverified. SMMUv1/v2 from offset 16: base
//! u64, span u64, model u32, flags u32, then the global interrupt array's
//! offset, the context interrupt count and offset, the PMU interrupt count
//! and offset, each an offset from the node's start to 8-byte entries
//! (GSIV, flags). RMR from 16: flags u32, the range count and offset, each
//! range 20 bytes (base u64, length u64, reserved u32). Every array must
//! lie inside the node; a range that is empty or wraps is no range.

use super::super::le::{u32_at, u64_at};
use super::kind::{Irq, Kind};
use super::node::HEADER;
use alloc::vec::Vec;

const IRQ: usize = 8;
const RANGE: usize = 20;
/// SMMU_NSgIrpt and SMMU_NSgCfgIrpt.
const GLOBAL: u32 = 2;

/// `count` entries of `size` bytes at `at`, all after the header and
/// inside the node; an empty array may sit anywhere.
fn entries(n: &[u8], at: u32, count: u32, size: usize) -> Option<Vec<&[u8]>> {
    if count == 0 {
        return Some(Vec::new());
    }
    let at = at as usize;
    let end = (count as usize).checked_mul(size)?.checked_add(at)?;
    let all = n.get(at..end).filter(|_| at >= HEADER)?;
    Some(all.chunks(size).collect())
}

fn irqs(n: &[u8], at: u32, count: u32) -> Option<Vec<Irq>> {
    let e = entries(n, at, count, IRQ)?;
    e.iter().map(|e| Some(Irq { gsiv: u32_at(e, 0)?, edge: u32_at(e, 4)? & 1 != 0 })).collect()
}

pub(crate) fn smmu_v2(n: &[u8]) -> Option<Kind> {
    Some(Kind::Smmuv2 {
        base: u64_at(n, 16)?,
        span: u64_at(n, 24)?,
        model: u32_at(n, 32)?,
        flags: u32_at(n, 36)?,
        global: irqs(n, u32_at(n, 40)?, GLOBAL)?,
        context: irqs(n, u32_at(n, 48)?, u32_at(n, 44)?)?,
        pmu: irqs(n, u32_at(n, 56)?, u32_at(n, 52)?)?,
    })
}

pub(crate) fn rmr(n: &[u8]) -> Option<Kind> {
    let e = entries(n, u32_at(n, 24)?, u32_at(n, 20)?, RANGE)?;
    let range = |e: &&[u8]| {
        let (base, len) = (u64_at(e, 0)?, u64_at(e, 8)?);
        (len != 0 && base.checked_add(len).is_some()).then_some((base, len))
    };
    let ranges = e.iter().map(range).collect::<Option<Vec<_>>>()?;
    Some(Kind::Rmr { flags: u32_at(n, 16)?, ranges })
}
