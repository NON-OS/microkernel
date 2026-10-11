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

//! Reading HST_STS (ICH and PCH datasheets, Host Status). HOST_BUSY is set
//! while a transaction runs; at its end exactly one of INTR (success),
//! DEV_ERR, BUS_ERR or FAILED is set, each cleared by writing it back.

use crate::errno::{E_AGAIN, E_IO, E_NXIO};
use crate::regs::{STS_BUS_ERR, STS_DEV_ERR, STS_ERROR, STS_FAILED, STS_HOST_BUSY, STS_INTR};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seen {
    /// Still running, or idle with no end recorded yet.
    Running,
    Finished,
    Fault(i32),
}

pub fn seen(st: u8) -> Seen {
    if st & STS_HOST_BUSY != 0 || st & (STS_ERROR | STS_INTR) == 0 {
        return Seen::Running;
    }
    match fault(st) {
        Some(e) => Seen::Fault(e),
        None => Seen::Finished,
    }
}

/// Should more than one error bit show, the one that says most about what
/// to do next is answered. A lost arbitration (BUS_ERR) means the bus was
/// never had, so asking again is right whatever else is set. DEV_ERR is a
/// device that did not acknowledge, or an SMBus timeout. FAILED follows
/// only a KILL, which this driver issues when its own wait runs out.
fn fault(st: u8) -> Option<i32> {
    [(STS_BUS_ERR, E_AGAIN), (STS_DEV_ERR, E_NXIO), (STS_FAILED, E_IO)]
        .into_iter()
        .find(|&(bit, _)| st & bit != 0)
        .map(|(_, e)| e)
}
