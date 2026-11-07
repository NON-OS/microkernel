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

//! What SERVE_ONE keeps once the kernel serves the box's listener: the
//! field, the wire's server, who owns the capsule's endpoint, and where a
//! frame is taken in. Only the serve loop reaches it, as with the table.

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_libc::ipc::mk_serve_one;
use nonos_linux_policy::ServeOne;
use nonos_serve_wire::{HEADER, MAX_BODY};

use super::refusal::Refusal;
use super::server::Server;
use crate::linux::abi::errno;
use crate::linux::net::sock::Addr;

pub(super) struct Served {
    pub(super) field: ServeOne,
    pub(super) server: Server,
    /// The pid last seen owning the capsule's endpoint (owner.rs).
    pub(super) owner: u32,
    /// When the opener was last looked for, in mk_uptime_ms milliseconds.
    pub(super) looked: u64,
    /// One byte past the largest frame, so a longer one reads long and is refused.
    pub(super) buf: Vec<u8>,
}

pub(super) struct One(pub(super) RefCell<Option<Served>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches it;
 * guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for One {}

pub(super) static SERVED: One = One(RefCell::new(None));

/// Whether a listener is served, so SERVE_ONE's pump reads the inbox. Held
/// borrowed only while that pump runs, which is serving too.
pub fn serving() -> bool {
    SERVED.0.try_borrow().map_or(true, |s| s.is_some())
}

/// The guest's first listen at `at`: the kernel registers
/// linux.serve.<capsule> for this process, held to the capsule and paired
/// with the listing the family started for. A later listen finds it served.
pub(super) fn listen(field: ServeOne, at: Addr) -> Result<(), u64> {
    if SERVED.0.borrow().is_some() {
        return Ok(());
    }
    match mk_serve_one(field.name().as_bytes(), &crate::linux::app::listing()) {
        port if port > 0 => {
            let (server, buf) = (Server::new(at), vec![0; HEADER + MAX_BODY + 1]);
            *SERVED.0.borrow_mut() = Some(Served { field, server, owner: 0, looked: 0, buf });
            Ok(())
        }
        rc => Err(refused(rc, field.name())),
    }
}

/// The guest's answer for MkServeOne's `rc`, said where the person reads it
/// when the kernel refused the listener itself (refusal.rs).
fn refused(rc: i64, capsule: &str) -> u64 {
    match super::refusal::of(rc) {
        /* The capsule served already, or this process serving one. */
        Refusal::InUse => errno::fail(errno::EADDRINUSE),
        Refusal::NoRoom => errno::fail(errno::ENOBUFS),
        Refusal::NotPaired => {
            let line = super::refusal::not_paired(capsule);
            crate::linux::boxed::note(crate::linux::boxed::of_needs("SERVE_ONE"), b"listen");
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            errno::fail(errno::EACCES)
        }
        Refusal::Refused => crate::linux::app::refuse("listen", "SERVE_ONE", errno::EACCES),
    }
}
