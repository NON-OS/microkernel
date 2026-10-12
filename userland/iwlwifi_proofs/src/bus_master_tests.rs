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

//! W4a: the driver sets its own Bus Master, as the kernel's K6 leaves it to.
//! The claim turns on Memory Space alone; Bus Master comes on only once the
//! chip woke on the attempt's grants; and a broker that refuses either write
//! gives everything back, Bus Master off before any grant.

use nonos_libc::{given_back, refuse_config, EPOCH};

use crate::claim::claim;
use crate::constants::{CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY};
use crate::grants::{init_or_release, Grants};
use crate::regs::Regs;

const GRANTS: Grants = Grants { device_id: 5, claim_epoch: EPOCH, mmio: 6, irq: 7, dma: 8 };

fn since(before: usize) -> Vec<(&'static str, u64)> {
    given_back()[before..].to_vec()
}

#[test]
fn the_claim_turns_on_memory_space_alone() {
    let before = given_back().len();
    assert_eq!(claim(5), Ok(EPOCH));
    assert_eq!(since(before), [("device_claim", 5), ("pci_command", 2)]);
    refuse_config(true);
    let before = given_back().len();
    assert_eq!(claim(5), Err("iwlwifi: memory space enable failed"));
    refuse_config(false);
    assert_eq!(since(before), [("device_claim", 5), ("pci_command", 2), ("device_release", 5)]);
}

#[test]
fn bus_master_refused_after_the_chip_woke_gives_everything_back() {
    let mut window = vec![0u32; 0x100 / 4];
    window[CSR_GP_CNTRL / 4] = GP_CNTRL_MAC_CLOCK_READY;
    refuse_config(true);
    let before = given_back().len();
    let got = init_or_release(Regs::new(window.as_mut_ptr() as u64), &GRANTS);
    refuse_config(false);
    assert_eq!(got.err(), Some("iwlwifi: bus master enable failed"));
    let all = [("pci_command", 6), ("pci_command", 2), ("dma_unmap", 8)];
    assert_eq!(since(before)[..3], all, "on, refused; off before the DMA goes");
    assert_eq!(since(before)[3..], [("irq_unbind", 7), ("mmio_unmap", 6), ("device_release", 5)]);
}
