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

//! Bus Master goes on only once the controller is seen stopped (and never on
//! a refusal, refusal_tests). The controller here is one a predecessor left,
//! its queues and host memory buffer armed (fake::left_as): with Bus Master
//! on before it is disabled and RDY falls, it writes the frames they name
//! ("dma").

use nonos_libc::{calls, command, MK_PCI_CMD_BUS_MASTER};

use super::bring_up::stopped;
use crate::discover::Found;
use crate::fake::{left_as, LEFT};

/// The controller bring-up is handed: BAR0 the size of the fake's block.
pub(super) const FOUND: Found =
    Found { device_id: 7, bar_size: 0x2000, vendor: 0x144d, device: 0xa80a, cache: false };

/// Where the first call `what` names sits in the record from `from` on.
pub(super) fn first(from: usize, what: impl Fn(&(&str, u64)) -> bool) -> Option<usize> {
    calls()[from..].iter().position(what)
}

pub(super) fn bus_master(c: &(&str, u64)) -> bool {
    c.0 == "pci_command" && c.1 as u16 & MK_PCI_CMD_BUS_MASTER != 0
}

#[test]
fn bus_master_goes_on_only_once_the_controller_is_seen_stopped() {
    // Running, killed in its own disable or mid-enable, or fatal.
    for left in LEFT {
        let (_controller, from) = (left_as(left), calls().len());
        let (device, epoch) = stopped(FOUND).expect("taken and stopped");
        let at = |name| first(from, |c: &(&str, u64)| c.0 == name);
        let (decode, gone, master) = (at("pci_command"), at("stopped"), first(from, bus_master));
        assert!(decode < gone && gone < master, "{left:?}: {:?}", &calls()[from..]);
        assert_eq!(at("dma"), None, "{left:?}: the old queues were written to");
        assert_eq!(command(), 0x406, "{left:?}: Memory Space, Bus Master, INTx Disable");
        assert_eq!((device.handles.irq_grant_id(), epoch), (0x20, 1));
    }
}
