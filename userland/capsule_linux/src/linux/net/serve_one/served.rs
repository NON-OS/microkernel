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
        /* The capsule served already, or this process serving one. */
        e if e == -errno::EBUSY => Err(errno::fail(errno::EADDRINUSE)),
        e if e == -errno::ENOMEM => Err(errno::fail(errno::ENOBUFS)),
        _ => Err(crate::linux::app::refuse("listen", "SERVE_ONE", errno::EACCES)),
    }
}
