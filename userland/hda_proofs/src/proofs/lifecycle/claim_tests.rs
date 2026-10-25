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

//! Bus mastering after the claim, on a controller its last owner left armed.
//!
//! An owner that dies without its teardown (a panic aborts, a kill ends it)
//! leaves its engines armed: CORBRUN, RIRBDMAEN, a stream's RUN and DPLBASE,
//! each at frames the kernel has freed since, its release clearing only Bus
//! Master. The claim used to turn bus mastering straight back on, before the
//! reset stopped those engines, and with no IOMMU they wrote into memory
//! that by then was someone else's.

use nonos_libc::{mk_pci_config_write, Call, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use crate::constants::{
    CORBCTL, CORBCTL_RUN, DPLBASE, DPLBASE_ENABLE, RIRBCTL, RIRBCTL_DMAEN, SDCTL_RUN,
};
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::lifecycle::attempt::attempt;
use crate::proofs::lifecycle::board::{board, DEVICE, LOW};
use crate::proofs::lifecycle::teardown::{sd_ctl, STREAMS};
use crate::proofs::lifecycle::teardown_log::{CORB, DPL, RIRB};

/// The command writes in `log` that name Bus Master.
pub fn bus_master_on(log: &[Call]) -> Vec<&Call> {
    let on = |c: &&Call| c.name == "pci_config_update" && c.arg as u16 & MK_PCI_CMD_BUS_MASTER != 0;
    log.iter().filter(on).collect()
}

#[test]
fn bus_mastering_comes_on_only_once_a_dead_owners_engines_are_stopped() {
    let b = board(vec![(0, sim_codec(&alc236_hp()))], &LOW);
    b.sim.bar.present8(CORBCTL as usize, CORBCTL_RUN);
    b.sim.bar.present8(RIRBCTL as usize, RIRBCTL_DMAEN);
    b.sim.bar.present32(DPLBASE as usize, 0x0010_0001);
    b.sim.bar.present8(sd_ctl(4), SDCTL_RUN);
    // The kernel's release cleared Bus Master, and only that.
    mk_pci_config_write(DEVICE, 0, MK_PCI_CFG_COMMAND, MK_PCI_CMD_MEMORY_SPACE);
    let (_, log) = attempt(&b);
    let r = &bus_master_on(&log).first().expect("bus mastering never came on").regs;
    for (s, ctl) in r[..STREAMS].iter().enumerate() {
        assert_eq!(ctl & SDCTL_RUN as u32, 0, "stream {s} running as bus mastering came on");
    }
    assert_eq!(r[CORB] & CORBCTL_RUN as u32, 0, "CORB running as bus mastering came on");
    assert_eq!(r[RIRB] & RIRBCTL_DMAEN as u32, 0, "RIRB running as bus mastering came on");
    assert_eq!(r[DPL] & DPLBASE_ENABLE, 0, "position buffer on as bus mastering came on");
}
