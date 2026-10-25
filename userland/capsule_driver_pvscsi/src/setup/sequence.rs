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

//! The bring-up: claim, memory decode and bus mastering with the pin
//! silenced, the register window, the DMA grant, the adapter reset and
//! handed its rings; then the targets scanned. Nothing is bound: every
//! interrupt is masked and the driver polls the completion ring.

use nonos_libc::{
    mk_device_claim, mk_device_release, mk_dma_map, mk_dma_unmap, mk_mmio_map, mk_mmio_unmap,
    mk_pci_config_write, DmaMapOut, MmioMapOut, MK_DMA_MAP_COHERENT, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};
use nonos_scsi_serve::Units;

use super::driver::{abandon, Driver};
use crate::adapter;
use crate::constants::layout::GRANT_BYTES;
use crate::constants::pci::MEM_SPACE;
use crate::constants::regs::{MAX_LUN, MAX_TARGET};
use crate::discover::Found;
use crate::hba::PvHba;
use crate::regs::Mmio;
use crate::{NAME, REPLY_ENDPOINT};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let release = |e: &'static str| {
        let _ = mk_device_release(dev.device_id);
        e
    };
    let bits = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    if mk_pci_config_write(dev.device_id, epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err(release("pci command setup failed"));
    }
    let mut win = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    if mk_mmio_map(dev.device_id, epoch, dev.bar as u32, 0, 0, MEM_SPACE, &mut win) < 0 {
        return Err(release("mmio map failed"));
    }
    if win.length < MEM_SPACE {
        let _ = mk_mmio_unmap(win.grant_id);
        return Err(release("mmio window short"));
    }
    let regs = Mmio::new(win.user_va, win.length as usize);
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(dev.device_id, epoch, GRANT_BYTES, MK_DMA_MAP_COHERENT, &mut dma) < 0 {
        abandon(&regs, dev.device_id, win.grant_id, None);
        return Err("dma map failed");
    }
    if dma.length < GRANT_BYTES {
        let _ = mk_dma_unmap(dma.grant_id);
        abandon(&regs, dev.device_id, win.grant_id, None);
        return Err("dma grant short");
    }
    if let Err(e) = adapter::start(&regs, dma.user_va, dma.device_addr) {
        abandon(&regs, dev.device_id, win.grant_id, Some(dma.grant_id));
        return Err(e);
    }
    let mut units =
        Units::new(PvHba::new(regs, dma.user_va, dma.device_addr), REPLY_ENDPOINT, NAME);
    units.scan(MAX_TARGET, MAX_LUN);
    Ok(Driver {
        units,
        device_id: dev.device_id,
        mmio_grant: win.grant_id,
        dma_grant: dma.grant_id,
    })
}
