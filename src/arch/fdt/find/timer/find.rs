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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::irq::gic_intid;
use crate::arch::fdt::parser::Fdt;

use super::compatible::NEEDLES;
use super::info::TimerInfo;

// The architected timer's interrupts as GIC INTIDs. The binding fixes the
// order as secure-phys / non-sec-phys / virt / hyp-phys; the first two are
// required and the rest are left zero when absent. A specifier that does not
// decode is an error rather than a zero, since a zero here would read as
// "absent" and quietly drop the tick.
pub fn find(fdt: &Fdt) -> Result<Option<TimerInfo>, FdtError> {
    let Some(found) = fdt.devices(NEEDLES).next() else {
        return Ok(None);
    };
    let (node, _) = found?;
    let mut intids = [0u32; 4];
    for (index, slot) in intids.iter_mut().enumerate() {
        match fdt.interrupt(&node, index)? {
            Some(spec) => *slot = gic_intid(spec.cells)?,
            None if index < 2 => return Ok(None),
            None => break,
        }
    }
    Ok(Some(TimerInfo {
        secure_phys_intid: intids[0],
        nonsecure_phys_intid: intids[1],
        virtual_intid: intids[2],
        hyp_phys_intid: intids[3],
        clock_frequency: match fdt.property(&node, b"clock-frequency")? {
            Some(rate) if rate.data.len() == 4 => Some(rate.u32()?).filter(|hz| *hz != 0),
            _ => None,
        },
    }))
}
