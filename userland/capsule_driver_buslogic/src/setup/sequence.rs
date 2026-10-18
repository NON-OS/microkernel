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

//! The bring-up: claim, bus mastering with the pin silenced, the port
//! window, one DMA grant below 4 GiB, the adapter reset and given its
//! mailboxes; then the targets scanned. Nothing is bound: the driver polls
//! the incoming mailbox.

use nonos_libc::{
    mk_device_claim, mk_device_release, mk_dma_map, mk_dma_unmap, mk_pci_config_write,
    mk_pio_grant, mk_pio_release, DmaMapOut, PioGrantOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32,
    MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
};

use super::driver::{abandon, Driver};
use crate::adapter;
use crate::constants::layout::GRANT_BYTES;
use crate::constants::pci::{PORTS, PORT_BAR};
use crate::constants::regs::{MAX_LUN, MAX_TARGET};
use crate::discover::Found;
use crate::hba::BtHba;
use crate::port::Port;
use crate::{Units, NAME, REPLY_ENDPOINT};

/// The mailboxes and every CCB field are 32-bit bus addresses.
const DMA32_CEILING: u64 = 1 << 32;

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
    // Bus Master for the mailboxes and the data; the pin silenced, as the
    // driver polls. I/O decode is the firmware's.
    let bits = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    if mk_pci_config_write(dev.device_id, epoch, MK_PCI_CFG_COMMAND, bits) < 0 {
        return Err(release("pci command setup failed"));
    }
    let mut ports = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(dev.device_id, epoch, PORT_BAR, 0, &mut ports) < 0 {
        return Err(release("port grant failed"));
    }
    if ports.port_count < PORTS {
        let _ = mk_pio_release(ports.grant_id);
        return Err(release("port window short"));
    }
    let port = Port::new(ports.grant_id);
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(
        dev.device_id,
        epoch,
        GRANT_BYTES,
        MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT,
        &mut dma,
    ) < 0
    {
        abandon(&port, dev.device_id, ports.grant_id, None);
        return Err("dma map failed");
    }
    let fits = dma.device_addr.checked_add(dma.length).is_some_and(|end| end <= DMA32_CEILING);
    if dma.length < GRANT_BYTES || !fits {
        let _ = mk_dma_unmap(dma.grant_id);
        abandon(&port, dev.device_id, ports.grant_id, None);
        return Err("dma grant unusable");
    }
    let host_id = match adapter::start(&port, dma.user_va, dma.device_addr) {
        Ok(id) => id,
        Err(e) => {
            abandon(&port, dev.device_id, ports.grant_id, Some(dma.grant_id));
            return Err(e);
        }
    };
    let hba = BtHba { port, va: dma.user_va, dev: dma.device_addr, host_id, dead: false };
    let mut units = Units::new(hba, REPLY_ENDPOINT, NAME);
    units.scan(MAX_TARGET, MAX_LUN);
    Ok(Driver {
        units,
        device_id: dev.device_id,
        pio_grant: ports.grant_id,
        dma_grant: dma.grant_id,
    })
}
