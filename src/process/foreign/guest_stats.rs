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

//! What the process table shows for a guest that the kernel would not see
//! on its own: the calls trapped to its supervisor, the pages that
//! supervisor put into it, and the name of the program it runs.

use alloc::string::String;
use core::sync::atomic::Ordering;

use crate::process::accounting::{self, Kind, Total};

/// A trapped call is one of the guest's syscalls, counted as the syscall
/// entry counts every other process's.
pub(super) fn called(pid: u32) {
    accounting::bump(pid, Kind::Syscall);
    accounting::bump_total(Total::Syscalls);
}

/// Pages a peer call mapped into or took out of the guest, kept in its
/// resident count. A page filled on first touch of a reservation is not
/// counted here.
pub(super) fn resident(pid: u32, gained: u64, lost: u64) {
    crate::process::with_process(pid, |pcb| {
        let mem = pcb.memory.lock();
        let _ = mem.resident_pages.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            Some(n.saturating_add(gained).saturating_sub(lost))
        });
    });
}

/// Name the guest, once an exec has answered, after the program it now runs.
pub(super) fn rename(pid: u32, name: String) {
    crate::process::with_process(pid, |pcb| *pcb.name.lock() = name);
}

/// The part of a guest's name after "foreign:", which a forked child takes as
/// a Linux child takes its parent's comm.
pub(super) fn comm_of(pid: u32) -> String {
    crate::process::with_process(pid, |pcb| {
        String::from(pcb.name.lock().strip_prefix("foreign:").unwrap_or("fork"))
    })
    .unwrap_or_else(|| String::from("fork"))
}
