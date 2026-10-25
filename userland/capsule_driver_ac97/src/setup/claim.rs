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

//! Claim, the PCI command bits, and a port grant on each window. A failing
//! step gives back what the earlier ones took.

use nonos_libc::{
    mk_device_claim, mk_device_release, mk_pci_config_update, mk_pio_grant, mk_pio_release,
    PioGrantOut, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::grants::Grants;
use crate::constants::pci::{NABM_BAR, NABM_MIN_PORTS, NAM_BAR, NAM_MIN_PORTS};

/// Bus Master for the engine's reads of the list and the ring; the broker
/// clears it on every release, so each attempt sets it. The pin is
/// silenced: the driver polls. I/O decode is the firmware's, and a window
/// it left off reads all ones, which the codec reset refuses.
pub const COMMAND_BITS: u16 = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
/// The bits the write sets: those, and Memory Space cleared. The function
/// decodes nothing in memory, the capsule holds no Mmio, and firmware often
/// leaves the bit on, which a write that only ORed the others in would keep.
pub const COMMAND_MASK: u16 = COMMAND_BITS | MK_PCI_CMD_MEMORY_SPACE;

pub fn claim(device_id: u64) -> Result<Grants, &'static str> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let back = |held: &[u64], e: &'static str| {
        for &g in held {
            let _ = mk_pio_release(g);
        }
        let _ = mk_device_release(device_id);
        e
    };
    if mk_pci_config_update(device_id, epoch, MK_PCI_CFG_COMMAND, COMMAND_MASK, COMMAND_BITS) < 0 {
        return Err(back(&[], "pci command setup failed"));
    }
    let nam = grant(device_id, epoch, NAM_BAR, NAM_MIN_PORTS)
        .ok_or_else(|| back(&[], "mixer port grant failed"))?;
    let nabm = grant(device_id, epoch, NABM_BAR, NABM_MIN_PORTS)
        .ok_or_else(|| back(&[nam], "bus master port grant failed"))?;
    Ok(Grants { device_id, epoch, nam, nabm })
}

fn grant(device_id: u64, epoch: u64, bar: u8, min: u16) -> Option<u64> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(device_id, epoch, bar, 0, &mut out) < 0 {
        return None;
    }
    if out.port_count < min {
        let _ = mk_pio_release(out.grant_id);
        return None;
    }
    Some(out.grant_id)
}
