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

//! The one ticket into app.linux.term.net for a run its box does not
//! widen: a market program whose policy grants NET_ANON, started from the
//! Terminal. The terminal role binds the policy, asks by MkLinuxRunNet for
//! itself, and exits with ASK_NET; its Terminal then asks for
//! `tool.linux.net` with the same argv, once (tool_capsules/linux_terminal.rs).

use spin::Mutex;

use super::role_term_net::TERMINAL_NET;
use super::roles::TERMINAL;
use super::term_ticket_rule::{admit, end_of, Ticket};
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::ERRNO_BUSY;

static TICKET: Mutex<Option<Ticket>> = Mutex::new(None);

/// `None` when `pid` is not the process the kernel registered as
/// app.linux.term. Otherwise 0 with the ticket written, or EBUSY when the
/// network role is already held or a ticket is waiting: one runs at a time.
pub fn asked_by_terminal_role(pid: u32) -> Option<i64> {
    if lookup_service(TERMINAL.name)?.pid != pid {
        return None;
    }
    let terminal = group_of(crate::process::get_parent_pid(pid).filter(|&p| p != 0)?);
    let argv = crate::process::with_process(pid, |pcb| pcb.argv.lock().clone())?;
    let mut slot = TICKET.lock();
    if slot.is_some() || lookup_service(TERMINAL_NET.name).is_some() {
        return Some(ERRNO_BUSY);
    }
    *slot = Some(Ticket { terminal, argv });
    Some(0)
}

/// Whether `caller` holds the ticket for `argv`; the ticket is spent either way.
pub fn take_term_ticket(caller: u32, argv: &[alloc::string::String]) -> bool {
    admit(&mut TICKET.lock(), group_of(caller), argv)
}

/// `pid` is exiting; a ticket its Terminal held is void.
pub fn end_tickets_of(pid: u32) {
    end_of(&mut TICKET.lock(), pid);
}

/// The process `pid` is a thread of, as the Terminal is one process however
/// many threads it asks from.
fn group_of(pid: u32) -> u32 {
    crate::process::with_process(pid, |pcb| pcb.thread_group_id())
        .filter(|&g| g != 0)
        .unwrap_or(pid)
}
