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

//! What a function offers for its reset (L78), read once at enumeration:
//! where its PCIe, Advanced Features and Power Management capabilities sit,
//! the methods they offer, the identity it must answer with again, and
//! whether it may reach memory past Bus Master off.

use super::bits_rule::AF_TP_CAP;
use super::method_rule::{binds_past_bus_master, offers, Offers};
use crate::drivers::pci::types::PciDevice;
use crate::drivers::pci::ConfigSpace;

const CAP_PM: u8 = 0x01;
const CAP_PCIE: u8 = 0x10;
const CAP_AF: u8 = 0x13;

/// Offsets are 0 where the function has no such capability.
#[derive(Clone, Copy, Debug, Default)]
pub(in crate::hardware::broker) struct ResetCaps {
    pub(in crate::hardware::broker) pcie: u8,
    pub(in crate::hardware::broker) devctl2: bool,
    /// Device Control 2 as firmware left it, whose completion timeout a put
    /// back restores: a network holder may change it (L80).
    pub(in crate::hardware::broker) devctl2_boot: u16,
    pub(in crate::hardware::broker) af: u8,
    pub(in crate::hardware::broker) af_tp: bool,
    pub(in crate::hardware::broker) pm: u8,
    pub(in crate::hardware::broker) offers: Offers,
    /// Vendor and device, as dword 0 reads.
    pub(in crate::hardware::broker) ident: u32,
    pub(in crate::hardware::broker) display: bool,
    pub(in crate::hardware::broker) binds: bool,
    pub(in crate::hardware::broker) mem_bar: bool,
}

pub(in crate::hardware::broker) fn reset_caps(dev: &PciDevice) -> ResetCaps {
    let cfg = ConfigSpace::new(dev.address);
    let at = |id: u8| dev.capabilities.iter().find(|c| c.id == id).map(|c| (c.offset, c.version));
    let (pcie, af, pm) = (at(CAP_PCIE), at(CAP_AF), at(CAP_PM));
    let devcap = pcie.and_then(|(o, _)| cfg.read32(u16::from(o) + 4).ok());
    let af_caps = af.and_then(|(o, _)| cfg.read8(u16::from(o) + 3).ok());
    let pmcsr = pm.and_then(|(o, _)| cfg.read16(u16::from(o) + 4).ok());
    let v2 = pcie.filter(|p| p.1 >= 2);
    let devctl2_boot = v2.and_then(|(o, _)| cfg.read16(u16::from(o) + 0x28).ok());
    let untranslated = super::super::hold_rule::untranslated(dev.vendor_id, dev.device_id);
    ResetCaps {
        pcie: pcie.map_or(0, |p| p.0),
        devctl2: v2.is_some(),
        devctl2_boot: devctl2_boot.unwrap_or(0),
        af: af.map_or(0, |a| a.0),
        af_tp: af_caps.is_some_and(|c| c & AF_TP_CAP != 0),
        pm: pm.map_or(0, |p| p.0),
        offers: offers(devcap, af_caps, pmcsr),
        ident: u32::from(dev.vendor_id) | u32::from(dev.device_id) << 16,
        display: dev.class == 0x03,
        binds: binds_past_bus_master(untranslated, dev.vendor_id, dev.class),
        mem_bar: dev.bars.iter().any(|b| b.address().is_some() && b.size() > 0),
    }
}
