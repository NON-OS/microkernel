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

//! Taking the unit from whatever firmware left to translating with the
//! kernel's tables, every stream aborting until it is given a domain.

use super::cmdq::Cmdq;
use super::encode::cmd;
use super::eventq::Eventq;
use super::mmio::Regs;
use super::regs::*;
use super::strtab::StreamTable;
use super::wait;

fn set_cr0(regs: Regs, value: u32) -> Result<(), &'static str> {
    regs.write32(CR0, value);
    if wait::until(|| regs.read32(CR0ACK) == value) {
        Ok(())
    } else {
        Err("the SMMUv3 did not acknowledge CR0")
    }
}

pub(super) fn reset(
    regs: Regs,
    q: &mut Cmdq,
    events: &Eventq,
    table: &StreamTable,
) -> Result<(), &'static str> {
    set_cr0(regs, 0)?;
    // While it is off, a transaction aborts rather than bypassing. Firmware
    // may have left bypass set; this is what holds DMA until the end.
    regs.write32(GBPA, GBPA_UPDATE | GBPA_ABORT);
    if !wait::until(|| regs.read32(GBPA) & GBPA_UPDATE == 0) {
        return Err("the SMMUv3 did not take GBPA");
    }
    regs.write32(CR1, CR1_VALUE);
    regs.write32(CR2, CR2_VALUE);
    regs.write64(STRTAB_BASE, table.base());
    regs.write32(STRTAB_BASE_CFG, table.config());
    regs.write64(CMDQ_BASE, q.base());
    regs.write32(CMDQ_PROD, 0);
    regs.write32(CMDQ_CONS, 0);
    regs.write64(EVENTQ_BASE, events.base());
    regs.write32(EVENTQ_PROD, 0);
    regs.write32(EVENTQ_CONS, 0);
    wait::publish();
    set_cr0(regs, CR0_CMDQEN)?;
    // Whatever firmware cached from its own tables goes before the
    // kernel's take effect.
    q.run(regs, &[cmd::cfgi_all(), cmd::tlbi_nsnh_all()])
        .map_err(|_| "the SMMUv3 did not complete its first commands")?;
    set_cr0(regs, CR0_CMDQEN | CR0_EVENTQEN)?;
    set_cr0(regs, CR0_CMDQEN | CR0_EVENTQEN | CR0_SMMUEN)?;
    // An error is active while its GERROR bit differs from GERRORN's.
    if regs.read32(GERROR) ^ regs.read32(GERRORN) != 0 {
        return Err("the SMMUv3 reports a global error after enabling");
    }
    Ok(())
}
