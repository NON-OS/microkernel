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
    mk_device_claim, mk_device_release, mk_pci_config_write, mk_pio_grant, mk_pio_release,
    PioGrantOut, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
};

use super::grants::{ChannelGrants, Grants};
use crate::constants::pci::{
    command_bar, control_bar, BUS_MASTER_BAR, BUS_MASTER_MIN_PORTS, CHANNELS, COMMAND_MIN_PORTS,
    CONTROL_MIN_PORTS,
};
use crate::discover::Found;

/// Bus Master for the engines' reads and writes of memory; the broker
/// clears it on every release, so each attempt sets it. The PCI pin is
/// silenced as well: the driver polls, and a native-mode channel would
/// otherwise share INTx with whatever else is on the line. I/O decode is the
/// firmware's: with it off the status floats high and no channel is used.
pub const COMMAND_BITS: u16 = MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;

pub fn claim(dev: &Found) -> Result<Grants, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let mut g =
        Grants { device_id: dev.device_id, epoch: epoch as u64, bm: 0, channels: [None; CHANNELS] };
    let back = |g: &Grants, e: &'static str| {
        for c in g.channels.iter().flatten() {
            let _ = mk_pio_release(c.ctl);
            let _ = mk_pio_release(c.cmd);
        }
        if g.bm != 0 {
            let _ = mk_pio_release(g.bm);
        }
        let _ = mk_device_release(g.device_id);
        e
    };
    if mk_pci_config_write(g.device_id, g.epoch, MK_PCI_CFG_COMMAND, COMMAND_BITS) < 0 {
        return Err(back(&g, "pci command setup failed"));
    }
    g.bm = match grant(&g, BUS_MASTER_BAR, BUS_MASTER_MIN_PORTS) {
        Some(out) => out.grant_id,
        None => return Err(back(&g, "bus master port grant failed")),
    };
    for c in (0..CHANNELS).filter(|&c| dev.channels[c]) {
        let Some(cmd) = grant(&g, command_bar(c), COMMAND_MIN_PORTS) else {
            return Err(back(&g, "command block port grant failed"));
        };
        let Some(ctl) = grant(&g, control_bar(c), CONTROL_MIN_PORTS) else {
            let _ = mk_pio_release(cmd.grant_id);
            return Err(back(&g, "control block port grant failed"));
        };
        g.channels[c] =
            Some(ChannelGrants { cmd: cmd.grant_id, ctl: ctl.grant_id, ctl_ports: ctl.port_count });
    }
    Ok(g)
}

fn grant(g: &Grants, bar: u8, min: u16) -> Option<PioGrantOut> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    if mk_pio_grant(g.device_id, g.epoch, bar, 0, &mut out) < 0 {
        return None;
    }
    if out.port_count < min {
        let _ = mk_pio_release(out.grant_id);
        return None;
    }
    Some(out)
}
