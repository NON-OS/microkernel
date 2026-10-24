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

use crate::arch::aarch64::gic::{enable_irq, irq_held_by, register_irq_handler};
use crate::arch::aarch64::timer::set_timer;

use super::handler::{timer_tick, TICK_PERIOD_NS};
use super::state::phys_intid;

pub fn install_on_cpu() -> Result<(), &'static str> {
    let intid = phys_intid();
    if intid == 0 {
        return Err("timer intid unset (DTB /timer node missing or unparsed)");
    }
    register_irq_handler(intid, timer_tick).map_err(|_| "gic timer irq registration failed")?;
    enable_irq(intid);
    set_timer(TICK_PERIOD_NS);
    Ok(())
}

/// The tick on a secondary. The timer's interrupt is a PPI, banked per CPU,
/// and its one handler slot was filled by the boot CPU in `install_on_cpu`;
/// registering it again failed as already claimed, which refused every
/// secondary at this step. A secondary checks the boot CPU's registration,
/// then enables its own PPI and arms its own timer.
///
/// "Its own" is where the enable lands. On a GICv3, `gic::enable_irq` sends an
/// INTID below 32 to `frames::this_cpu()`, the redistributor frame this CPU
/// recorded in `init_gic_cpu`, and sets GICR_ISENABLER0 in that frame's SGI
/// page (RD_base + 0x10000); a GICD write for an SGI or PPI is RAZ/WI under
/// affinity routing. On a GICv2 the write is GICD_ISENABLER0, which is banked
/// per CPU. `aarch64_ap_entry` calls this only after `init_gic_cpu` returned
/// Ok, so the frame is recorded.
pub fn arm_on_secondary() -> Result<(), &'static str> {
    let intid = phys_intid();
    if intid == 0 {
        return Err("timer intid unset (DTB /timer node missing or unparsed)");
    }
    if !irq_held_by(intid, timer_tick) {
        return Err("the boot CPU's timer tick is not registered");
    }
    enable_irq(intid);
    set_timer(TICK_PERIOD_NS);
    Ok(())
}
