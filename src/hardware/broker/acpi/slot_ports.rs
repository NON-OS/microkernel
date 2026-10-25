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

//! The ports MkAcpiSlot serves: bridges the kernel enumerated whose PCI
//! Express capability, found here, passes slot_rule::eligible; kept from
//! the last PORT_AT for port 0, so a poll's access walks nothing.

use super::osc_grant::NATIVE_HOT_PLUG;
use super::osc_store::osc_grant;
use super::slot_access::config;
use super::slot_rule::eligible;
use super::slot_wire::Bdf;
use crate::drivers::pci::capabilities::find_capability;
use crate::drivers::pci::manager::with_manager;
use crate::drivers::pci::types::PciDevice;
use alloc::vec::Vec;

const PCIE: u8 = 0x10;
const LINK_CAPS: u16 = 0x0C;
const SLOT_CAPS: u16 = 0x14;
const DLL_REPORTING: u32 = 1 << 20;
const MAX_PORTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Port {
    pub(super) bdf: Bdf,
    pub(super) cap: u16,
    pub(super) dll: bool,
}

static PORTS: spin::Mutex<Vec<Port>> = spin::Mutex::new(Vec::new());

fn port(bus: u8, dev: u8, func: u8) -> Option<Port> {
    let cap = u16::from(find_capability(bus, dev, func, PCIE)?.offset);
    let cfg = config((0, bus, dev, func));
    // Offset 2 holds the port type and Slot Implemented.
    let (flags, slot) = (cfg.read16(cap + 2).ok()?, cfg.read32(cap + SLOT_CAPS).ok()?);
    let owns = osc_grant(0, bus).is_some_and(|g| g.os_owns(NATIVE_HOT_PLUG));
    if !eligible(flags, slot, owns) {
        return None;
    }
    let dll = cfg.read32(cap + LINK_CAPS).ok()? & DLL_REPORTING != 0;
    Some(Port { bdf: (0, bus, dev, func), cap, dll })
}

/// The `n`th port, walking enumeration afresh when `n` is 0.
pub(super) fn port_at(n: usize) -> Option<Port> {
    let mut ports = PORTS.lock();
    if n == 0 {
        let at = |d: &PciDevice| (d.bus(), d.device(), d.function());
        let bridges: Vec<(u8, u8, u8)> =
            with_manager(|m| m.find_bridges().into_iter().map(at).collect()).unwrap_or_default();
        let found = bridges.into_iter().filter_map(|(b, d, f)| port(b, d, f));
        *ports = found.take(MAX_PORTS).collect();
    }
    ports.get(n).copied()
}

/// A port the last walk found; anything else is not MkAcpiSlot's.
pub(super) fn known(f: Bdf) -> Option<Port> {
    PORTS.lock().iter().find(|p| p.bdf == f).copied()
}
