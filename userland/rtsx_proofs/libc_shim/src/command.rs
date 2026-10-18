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

//! The PCI command write. It stores exactly the bits it names, as the
//! kernel does once K6 lands (plan/54), and gives the device a turn after
//! it: the moment bus mastering comes on is when an engine left armed would
//! run. A test can have the broker refuse it.

use std::cell::Cell;

use crate::claim::set_bus_master;
use crate::events::note;
use crate::hook::device_acts;
use crate::types::{MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER};

thread_local! {
    static REFUSE_MASTER: Cell<bool> = const { Cell::new(false) };
    static REFUSE_ALL: Cell<bool> = const { Cell::new(false) };
}

/// From now on the broker refuses a command write that sets Bus Master.
pub fn refuse_bus_master() {
    REFUSE_MASTER.with(|r| r.set(true));
}

/// From now on the broker refuses every command write.
pub fn refuse_command() {
    REFUSE_ALL.with(|r| r.set(true));
}

pub fn mk_pci_config_write(_device_id: u64, _epoch: u64, offset: u32, value: u16) -> i64 {
    let command = offset == MK_PCI_CFG_COMMAND;
    let master = command && value & MK_PCI_CMD_BUS_MASTER != 0;
    let refused = REFUSE_ALL.with(|r| r.get()) || master && REFUSE_MASTER.with(|r| r.get());
    if command && refused {
        note(format!("pci {offset:#x}={value:#06x} refused"));
        return -1;
    }
    note(format!("pci {offset:#x}={value:#06x}"));
    if command {
        set_bus_master(master);
        device_acts();
    }
    0
}
