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

//! The kernel's boot pass leaves Memory Space on for every function, and the
//! broker refuses a Command write that keeps it on for a capsule with no
//! Mmio, as the shim does. Each write the driver makes names the bit off.

use nonos_libc::MK_PCI_CMD_MEMORY_SPACE as MEM;
use nonos_libc::{command, set_command, MK_PCI_CMD_BUS_MASTER as BM};

use crate::part::DEVICE;
use crate::setup::command as write;

const IO_SPACE: u16 = 1;

#[test]
fn the_claim_time_write_turns_memory_space_off_with_bus_master() {
    set_command(IO_SPACE | MEM | BM);
    write(DEVICE, 1, false).expect("taken from the boot pass's register");
    assert_eq!(command(), IO_SPACE, "I/O decode kept, the other two off");
}

#[test]
fn bus_master_goes_on_with_memory_space_kept_off() {
    set_command(IO_SPACE | MEM);
    write(DEVICE, 1, true).expect("taken");
    assert_eq!(command(), IO_SPACE | BM);
}

#[test]
fn the_model_refuses_a_write_that_leaves_memory_space_on() {
    set_command(MEM);
    let cfg = nonos_libc::MK_PCI_CFG_COMMAND;
    assert!(nonos_libc::mk_pci_config_update(DEVICE, 1, cfg, BM, BM) < 0);
    assert_eq!(command(), MEM, "left as it was");
}
