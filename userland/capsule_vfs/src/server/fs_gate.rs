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

//! Who vfs answers.
//!
//! The store holds every file the person keeps and the packages the next
//! boot trusts. FileSystem is the bit the kernel grants for it, and vfs
//! served any process that found its port, so the bit decided nothing. vfs
//! now answers the kernel-internal client, which arrives as sender pid 0,
//! and otherwise only a sender the kernel says holds FileSystem. The kernel
//! is asked on every request: a cached verdict would outlive the holder's
//! exit and follow its pid to whatever process is handed that pid next.

pub(super) mod guard;
mod paths;
mod reach;
mod rule;

use alloc::vec::Vec;

use nonos_libc::mk_cap_check;

use super::handlers::path::normalize;
use crate::protocol::{encode_response, Request, EACCES};

/// FileSystem, as abi/caps.toml numbers it.
pub const CAP_FILE_SYSTEM: u64 = 1 << 6;
/// ForeignExec, which only capsule_linux holds (guard.rs).
pub const CAP_FOREIGN_EXEC: u64 = 1 << 32;

/// The reply that refuses `req`, or None when its sender is served.
pub fn refusal(req: &Request<'_>, sender_pid: u32) -> Option<Vec<u8>> {
    if rule::allows(sender_pid, |pid| mk_cap_check(pid, CAP_FILE_SYSTEM))
        && reaches(req, sender_pid)
    {
        return None;
    }
    Some(encode_response(req.op, req.flags, req.request_id, EACCES, &[]))
}

/// Whether the paths `req` names are its caller's to reach (guard.rs). The
/// caller is the sender, or for the kernel's own client the pid it sends
/// for, as `split_caller` reads it; the kernel itself (0) reaches all.
fn reaches(req: &Request<'_>, sender_pid: u32) -> bool {
    let Some((pid, rest)) = req.payload.split_first_chunk::<4>() else { return true };
    let caller = if sender_pid == 0 { u32::from_le_bytes(*pid) } else { sender_pid };
    reach::reaches(req.op, rest, normalize, || {
        caller == 0 || mk_cap_check(caller, CAP_FOREIGN_EXEC)
    })
}
