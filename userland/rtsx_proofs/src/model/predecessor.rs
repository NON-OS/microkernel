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

//! What an owner that died without its teardown (a panic, a kill) leaves
//! the next: a read in flight, its command buffer waiting at its check and
//! ADMA armed at a scatter-gather entry, both in frames the kernel freed
//! when it released the reader. The release cleared Bus Master; nothing
//! stopped the engines. The device takes its turn at each PCI command
//! write, and once bus mastering is on whatever is still armed runs: what
//! it reaches outside every live grant is a violation.

use nonos_libc::{bus_master, note, set_device_hook};

use super::knobs::Phase;
use super::linux::HDBCTLR_TO_HOST;
use super::reader::with;
use super::run::Run;
use crate::regs::host::TRIG_DMA;
use crate::regs::sd::{SD_TRANSFER, SD_TRANSFER_END};
use crate::wire::{CmdBuf, CmdKind};

/// Where the dead owner's command buffer and scatter-gather table were.
const OLD_HCBAR: u32 = 0x0FE0_0000;
const OLD_HDBAR: u32 = 0x0FF0_0000;

pub fn left_armed() {
    let mut buf = CmdBuf::new();
    buf.add(CmdKind::Check, SD_TRANSFER, SD_TRANSFER_END, SD_TRANSFER_END);
    with(|r| {
        (r.hcbar, r.hdbar, r.data) = (OLD_HCBAR, OLD_HDBAR, Some(Phase::Read));
        r.run = Some(Run::new(buf.words().to_vec(), true));
    });
    set_device_hook(|| {
        with(|r| {
            if !bus_master() {
                return;
            }
            let (cmd, dma) = (r.run.is_some(), r.data.is_some());
            let (cmd_stopped, dma_stopped) = (r.cmd_stopped, r.dma_stopped);
            note(format!(
                "bus master on: cmd armed {cmd} stopped {cmd_stopped}; \
                 dma armed {dma} stopped {dma_stopped}"
            ));
            if dma {
                r.adma(TRIG_DMA | HDBCTLR_TO_HOST);
            }
            if let Some(run) = r.run.take() {
                r.resume(run);
            }
        })
    });
}
