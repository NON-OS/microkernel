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

//! Configuration space as a tg3 function holds it before bring-up, and
//! every write a run made replayed through the kernel broker's rule: the
//! Command bits it allows, then quirk_bits for the vendor registers.

use nonos_libc::{config_set, config_writes, mk_pci_config_read};

use crate::constants::pcicfg::*;
use crate::quirk_bits::{clear_only, cleared_only, only, writable, Ident};

use super::build::PCIE_CAP;

const COMMAND: u32 = 0x04;
/// Memory Space, Bus Master and INTx Disable (allowlist.rs).
const COMMAND_BITS: u16 = 0x0406;

/// Memory Space and Bus Master on, host control's revision nibble sending
/// tg3 to the product-ID register as every 57765-class part's does, and
/// Relaxed Ordering and No Snoop on as a reset leaves them.
pub fn seed() {
    config_set(COMMAND, 0x0006, 2);
    config_set(TG3PCI_MISC_HOST_CTRL, 0xF000_0002, 4);
    config_set(TG3PCI_PCISTATE, 0x0000, 2);
    config_set(PCIE_CAP + PCI_EXP_DEVCTL, 0x2810, 2);
    config_set(PCIE_CAP + PCI_EXP_DEVSTA, 0x0005, 2);
}

/// Panics on the first write the broker would refuse; returns how many.
pub fn replay_against_the_broker(start: &[(u32, u16)]) -> usize {
    let tg3 =
        Ident { vendor: 0x14E4, class: 0x02, subclass: 0x00, pcie_cap: Some(PCIE_CAP as u16) };
    let mut space: Vec<(u32, u16)> = start.to_vec();
    let writes = config_writes();
    for &(at, new) in &writes {
        let cur = space.iter().rev().find(|(o, _)| *o == at).map(|(_, v)| *v).unwrap();
        let ok = if at == COMMAND {
            only(COMMAND_BITS, new, cur)
        } else if let Some(mask) = writable(&tg3, at) {
            only(mask, new, cur)
        } else if let Some(mask) = clear_only(&tg3, at) {
            cleared_only(mask, new, cur)
        } else {
            false
        };
        assert!(ok, "broker refuses {new:#06x} at {at:#x} over {cur:#06x}");
        space.push((at, new));
    }
    writes.len()
}

/// The 16-bit words seed() left, for replay to start from.
pub fn seeded() -> Vec<(u32, u16)> {
    let at = [COMMAND, TG3PCI_MISC_HOST_CTRL, TG3PCI_PCISTATE];
    let caps = [PCIE_CAP + PCI_EXP_DEVCTL, PCIE_CAP + PCI_EXP_DEVSTA];
    at.iter().chain(caps.iter()).map(|&o| (o, mk_pci_config_read(9, 1, o, 2) as u16)).collect()
}
