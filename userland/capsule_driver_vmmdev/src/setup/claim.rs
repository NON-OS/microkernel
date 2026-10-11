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

//! Claim, the command bits, the port grant, the event page and the request
//! page. A failing step gives back what the earlier ones took.

use nonos_libc::{
    mk_device_claim, mk_mmio_map, mk_pci_config_write, mk_pio_grant, DmaMapOut, MmioMapOut,
    PioGrantOut, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::grants::Grants;
use super::page::request_page;
use crate::constants::pci::{MEMORY_BAR, MEMORY_MAP_BYTES, PORT_BAR};

/// Memory decode for the event page, Bus Master for the request page, the
/// pin silenced: the driver polls the event page instead.
pub const COMMAND_BITS: u16 =
    MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;

pub struct Held {
    pub grants: Grants,
    pub port: u64,
    pub memory: MmioMapOut,
    pub page: DmaMapOut,
}

pub fn claim(device_id: u64) -> Result<Held, &'static str> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let mut g = Grants { device_id, ..Grants::default() };
    let back = |g: &Grants, e: &'static str| {
        g.release();
        e
    };
    if mk_pci_config_write(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_BITS) < 0 {
        return Err(back(&g, "pci command setup failed"));
    }
    let mut pio = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(device_id, epoch, PORT_BAR, 0, &mut pio) < 0 {
        return Err(back(&g, "port grant failed"));
    }
    g.pio = Some(pio.grant_id);
    if pio.port_count < 4 {
        return Err(back(&g, "request port too small"));
    }
    let mut memory = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(device_id, epoch, MEMORY_BAR, 0, 0, MEMORY_MAP_BYTES, &mut memory);
    if r < 0 {
        return Err(back(&g, "event page map failed"));
    }
    g.mmio = Some(memory.grant_id);
    let page = request_page(device_id, epoch).ok_or_else(|| back(&g, "request page map failed"))?;
    g.dma = Some(page.grant_id);
    Ok(Held { grants: g, port: pio.grant_id, memory, page })
}
