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

//! Bringing one VMD's domain up: map CFGBAR, find where the child buses
//! start, number them and place the children's memory, then describe the
//! domain so its config space can be reached by segment.

use super::super::config::ConfigSpace;
use super::super::types::{PciAddress, PciDevice};
use super::cfgbar::Cfgbar;
use super::domain::{assign, bus_count, bus_start, pick_window, VMCAP, VMCONFIG};
use super::membar::bar;
use super::registry::VmdDomain;
use super::report::{announce, hidden, report};
use crate::memory::addr::PhysAddr;

const CMD_MEM: u16 = 1 << 1;
const CMD_MASTER: u16 = 1 << 2;

pub(super) fn bring_up(dev: &PciDevice, segment: u16) -> Option<VmdDomain> {
    announce(dev);
    let config = ConfigSpace::new(dev.address);
    let Ok(command) = config.command() else {
        hidden(b"command register unreadable");
        return None;
    };
    let _ = config.set_command(command | CMD_MEM | CMD_MASTER);

    let (cfg_base, cfg_size) = bar(&dev.bars, 0);
    if cfg_base == 0 || cfg_size < (1 << 20) {
        hidden(b"CFGBAR missing");
        return None;
    }
    let vmcap = config.read16(VMCAP).unwrap_or(0);
    let vmconfig = config.read16(VMCONFIG).unwrap_or(0);
    let Some(first) = bus_start(dev.device_id, vmcap, vmconfig) else {
        hidden(b"reserved bus range in VMCONFIG");
        return None;
    };
    let buses = bus_count(first, cfg_size);
    let Some(window) = pick_window(bar(&dev.bars, 2), bar(&dev.bars, 4)) else {
        hidden(b"no MEMBAR to place the drives in");
        return None;
    };
    let span = (buses as usize) << 20;
    let Ok(va) = crate::memory::mmio::map_device_memory(PhysAddr::new(cfg_base), span) else {
        hidden(b"CFGBAR could not be mapped");
        return None;
    };
    let mut port = Cfgbar { va: va.as_u64(), bus_start: first, bus_count: buses };
    let done = assign(&mut port, first, buses, window);
    report(segment, first, &done);
    Some(VmdDomain {
        segment,
        vmd: PciAddress::new(dev.address.bus, dev.address.device, dev.address.function),
        cfg_va: va.as_u64(),
        bus_start: first,
        bus_count: buses,
    })
}
