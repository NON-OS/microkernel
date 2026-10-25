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

//! Where a service's answer to a pending mk_ipc_call may land, pure. The
//! inbox comes from the kernel's own pending entry, written when the caller
//! called, so the service never names it; what is checked is that the entry
//! still names its caller. kernel_proofs::syscall::microkernel::reply_dest.

/// Who the inbox registry says owns the inbox.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Owner {
    /// Not registered.
    Missing,
    /// A spawn's reply inbox, which the kernel holds for the process.
    Kernel,
    Pid(u32),
}

/// The kernel's facts at the moment of the answer.
pub(crate) struct Facts<'a> {
    /// The inbox the pending entry recorded.
    pub(crate) recorded: &'a str,
    /// The inbox the caller's pid names now.
    pub(crate) names_now: &'a str,
    /// Whether that name is the process's own reply inbox, not `proc.<pid>`.
    pub(crate) named: bool,
    pub(crate) owner: Owner,
    pub(crate) caller: u32,
    /// The caller is in the table and neither a zombie nor terminated.
    pub(crate) alive: bool,
    /// The service's peer list lets it reach the caller.
    pub(crate) reachable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Refused {
    CallerGone,
    NotItsInbox,
    NotItsOwner,
    Unreachable,
}

pub(crate) fn admit(f: &Facts) -> Result<(), Refused> {
    if !f.alive {
        return Err(Refused::CallerGone);
    }
    if f.names_now != f.recorded {
        return Err(Refused::NotItsInbox);
    }
    let owned = match f.owner {
        Owner::Pid(p) => p == f.caller,
        Owner::Kernel => f.named,
        Owner::Missing => false,
    };
    if !owned {
        return Err(Refused::NotItsOwner);
    }
    if !f.reachable {
        return Err(Refused::Unreachable);
    }
    Ok(())
}
