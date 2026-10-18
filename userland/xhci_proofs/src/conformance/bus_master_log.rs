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

//! The bring-up log read for bus mastering: when it first went on, what
//! the controller showed then (`bus_master_seen`), and whether the claim was
//! given back.

use nonos_libc::{Call, Logged, MK_PCI_CMD_BUS_MASTER};

use super::bus_master_seen::{RESET, RUNNING};

fn bus_master_on(l: &Logged) -> bool {
    matches!(l.call, Call::PciCommand(v) if v & MK_PCI_CMD_BUS_MASTER != 0)
}

/// What the controller showed when bus mastering first went on, if it did.
pub fn first_bus_master_on(log: &[Logged]) -> Option<u64> {
    log.iter().find(|l| bus_master_on(l)).map(|l| l.seen)
}

/// Whether a call from the reset to bus mastering going on saw RUN set.
pub fn ran_before_bus_master(log: &[Logged]) -> bool {
    let at = log.iter().position(bus_master_on).unwrap_or(log.len());
    log[..at].iter().any(|l| l.seen & (RESET | RUNNING) == RESET | RUNNING)
}

/// Whether the claim on the bring-up model's device was given back.
pub fn released(log: &[Logged]) -> bool {
    log.iter().any(|l| l.call == Call::DeviceRelease(7))
}
