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

//! What every way out of a bring-up must leave in the broker log, read off
//! it call by call: no DMA grant given back by hand, and a command write
//! with Bus Master clear made while every engine read stopped, the position
//! buffer was off and the controller was in reset; then the interrupt and
//! the register window, which hold no frame; and the release last.

use nonos_libc::{Call, MK_PCI_CMD_BUS_MASTER};

use crate::constants::{CORBCTL_RUN, DPLBASE_ENABLE, GCTL_CRST, RIRBCTL_DMAEN, SDCTL_RUN};
use crate::proofs::lifecycle::teardown::STREAMS;
use crate::proofs::lifecycle::teardown_log::{release, CMD, CORB, DPL, GCTL_AT, RIRB};

/// The end of every teardown, last call first.
const TAIL: [&str; 5] =
    ["device_release", "mmio_unmap", "irq_unbind", "pci_config_update", "pci_config_read"];

pub fn quiesced_then_released(log: &[Call]) {
    let by_hand: Vec<&Call> = log.iter().filter(|c| c.name == "dma_unmap").collect();
    assert!(by_hand.is_empty(), "DMA given back ahead of the release: {by_hand:?}");
    let last = release(log);
    let tail: Vec<&str> = log.iter().rev().take(TAIL.len()).map(|c| c.name).collect();
    assert_eq!(tail, TAIL, "{log:?}");
    let write = &log[log.len() - 4];
    assert_eq!(write.arg as u16 & MK_PCI_CMD_BUS_MASTER, 0, "Bus Master written on");
    let r = &write.regs;
    for (s, ctl) in r[..STREAMS].iter().enumerate() {
        assert_eq!(ctl & SDCTL_RUN as u32, 0, "stream {s} running at the command write");
    }
    assert_eq!(r[CORB] & CORBCTL_RUN as u32, 0, "CORB engine running at the command write");
    assert_eq!(r[RIRB] & RIRBCTL_DMAEN as u32, 0, "RIRB engine running at the command write");
    assert_eq!(r[DPL] & DPLBASE_ENABLE, 0, "position buffer on at the command write");
    assert_eq!(r[GCTL_AT] & GCTL_CRST, 0, "controller out of reset at the command write");
    assert_eq!(last.regs[CMD] as u16 & MK_PCI_CMD_BUS_MASTER, 0, "Bus Master on at release");
}
