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

//! Every `iommu-addresses` entry under `/reserved-memory`.

use crate::arch::fdt::endian::{be_cells, be_u32};
use crate::arch::fdt::parser::Fdt;
use crate::memory::iommu::reserved::ReservedRange;

use super::entry::{TreeRange, MAX_TREE_RANGES};
use super::owner::owner;
use super::refusal::Refusal;

/// Each entry `<&device iova size>`, the address and size in the cells of
/// the device's own bus, kept with the child's first `reg` as its physical
/// base. An entry whose device cannot be keyed is not kept: its reason goes
/// to `unkeyed` (the first one), for boot to say.
pub fn ranges(
    fdt: &Fdt,
    out: &mut [TreeRange; MAX_TREE_RANGES],
    unkeyed: &mut Option<&'static [u8]>,
) -> Result<usize, Refusal> {
    let Some(parent) = fdt.find_path(b"/reserved-memory")? else {
        return Ok(0);
    };
    let mut filled = 0usize;
    for child in fdt.children(&parent)? {
        let child = child?;
        let Some(prop) = fdt.property(&child, b"iommu-addresses")? else {
            continue;
        };
        let region = fdt.mmio(&child, 0)?;
        let mut at = 0usize;
        while at < prop.data.len() {
            let device = fdt.by_phandle(be_u32(prop.data, at)?)?;
            let (a, s) = (device.cells.address as usize, device.cells.size as usize);
            // On a PCI bus: phys.hi, then phys.mid:lo, the IOVA (PCI bus
            // binding, recalled).
            let iova = match a {
                3 => be_cells(prop.data, at + 8, 2)?,
                _ => be_cells(prop.data, at + 4, a as u32)?,
            };
            let len = be_cells(prop.data, at + 4 + 4 * a, s as u32)?;
            at += 4 + 4 * (a + s);
            let owner = match owner(fdt, &device)? {
                Ok(owner) => owner,
                Err(why) => {
                    unkeyed.get_or_insert(why);
                    continue;
                }
            };
            // A child without `reg` asks only that the IOVA be kept free,
            // which a reserved range (iova, phys, length) cannot carry.
            let problem = region.is_none().then_some(&b"an IOVA hole with no physical range"[..]);
            let phys = region.map_or(0, |(base, _)| base);
            let range = ReservedRange { iova, phys, len };
            *out.get_mut(filled).ok_or(Refusal::TooMany)? = TreeRange { owner, range, problem };
            filled += 1;
        }
    }
    Ok(filled)
}
