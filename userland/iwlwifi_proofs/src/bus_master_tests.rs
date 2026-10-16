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
//! The claim turns on Memory Space alone, and setup never turns Bus Master
//! on: that waits for the device's own master to read back stopped, right
//! before the firmware boots (gen3_master_tests).

use nonos_libc::{given_back, refuse_config, EPOCH};

use crate::claim::claim;
use crate::constants::{CSR_GP_CNTRL, GP_CNTRL_MAC_CLOCK_READY};
use crate::grants::{init_or_release, Grants};
use crate::regs::Regs;

fn grants() -> Grants {
    Grants { device_id: 5, claim: Some(EPOCH), mmio: Some(6), irq: Some(7) }
}

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
fn setup_wakes_the_chip_and_never_turns_bus_master_on() {
    let mut window = vec![0u32; 0x100 / 4];
    window[CSR_GP_CNTRL / 4] = GP_CNTRL_MAC_CLOCK_READY;
    let before = given_back().len();
    // SAFETY: the window's own 0x100 bytes, alive for the call.
    let regs = unsafe { Regs::new(window.as_mut_ptr() as u64, 0x100) };
    assert!(init_or_release(regs, &mut grants()).is_ok());
    assert!(since(before).is_empty(), "no command write: Bus Master waits for the stop");
}
