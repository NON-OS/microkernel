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

//! Bringing the ITS up: LPIs on at the boot CPU's redistributor, the ITS's
//! tables and queue given to it while it is off, then one collection mapped
//! to that redistributor, which every LPI is delivered through.

use super::super::distributor::GicDistributor;
use super::super::frames;
use super::super::intid::{LPI_BASE, LPI_COUNT};
use super::super::state::dist_base;
use super::encode::route::{mapc, target};
use super::encode::typer::decode;
use super::queue::Queue;
use super::regs::{Regs, CTLR, CTLR_ENABLED, TRANSLATER, TYPER};
use super::state::{Its, COLLECTION, EVENT_BITS, ITS, LPI_WORDS, MAX_DEVICES};
use super::{lpi_tables, tables};

/// INTID bits the LPI tables describe: 16383 is past the last LPI handed out.
const ID_BITS: u32 = 14;

/// Bring up the ITS at `base` for device ids below `device_ids`, and return
/// the physical address of the doorbell a device writes its events to.
pub fn bring_up(base: u64, device_ids: u64) -> Result<u64, &'static str> {
    if ITS.lock().is_some() {
        return Err("an ITS is already in service");
    }
    let regs = Regs(base);
    let typer = decode(regs.read64(TYPER));
    if !typer.physical {
        return Err("the ITS translates to no physical LPIs");
    }
    let id_bits = GicDistributor::new(dist_base()).lpi_id_bits().ok_or("the GIC has no LPIs")?;
    let id_bits = id_bits.min(ID_BITS);
    if 1u64 << id_bits < u64::from(LPI_BASE + LPI_COUNT) {
        return Err("the GIC's LPIs stop short of the kernel's");
    }
    let rd = frames::this_cpu().ok_or("the boot CPU has no redistributor frame")?;
    regs.quiesce()?;
    let lpi = lpi_tables::bring_up(&rd, id_bits)?;
    let device_limit = tables::program(regs, &typer, device_ids)?;
    let target = target(typer.pta, rd.base(), rd.processor_number());
    let mut queue = Queue::new(regs, target)?;
    regs.write32(CTLR, regs.read32(CTLR) | CTLR_ENABLED);
    queue
        .run(regs, &[mapc(COLLECTION, target)])
        .map_err(|_| "the ITS did not map its collection")?;
    let event_bits = typer.event_bits.min(EVENT_BITS);
    let devices = [None; MAX_DEVICES];
    let its =
        Its { regs, queue, typer, event_bits, device_limit, lpi, devices, lpis: [0; LPI_WORDS] };
    *ITS.lock() = Some(its);
    Ok(base + TRANSLATER)
}
