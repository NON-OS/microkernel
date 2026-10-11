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

//! The PCI command step: Bus Master, memory decode and the silenced pin in
//! one write to the command register under the claim's epoch, and a
//! refusal that gives the claim back.

use crate::setup::pci::{enable, COMMAND_BITS};
use nonos_libc::{
    given_back, pci_writes, script_pci, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

#[test]
fn bus_master_and_memory_decode_are_turned_on_under_the_claim() {
    script_pci(0);
    assert!(enable(7, 3).is_ok());
    assert_eq!(pci_writes(), vec![(7, 3, MK_PCI_CFG_COMMAND, COMMAND_BITS)]);
    for bit in [MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE, MK_PCI_CMD_INTX_DISABLE] {
        assert_ne!(COMMAND_BITS & bit, 0);
    }
    assert!(given_back().is_empty());
}

#[test]
fn a_refused_command_write_gives_the_claim_back() {
    script_pci(-1);
    assert!(enable(7, 3).is_err());
    assert_eq!(given_back(), vec![("device_release", 7)]);
}
