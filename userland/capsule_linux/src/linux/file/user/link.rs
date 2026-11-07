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

//! The broker, asked over IPC.
//!
//! One client per process, made at first use. Every request carries this
//! process's pid as the kernel stamps it; the broker asks the kernel whether
//! the sender holds ForeignExec, which only the Linux personality holds, so
//! this module is what stands between a guest and the person's files: it
//! asks only for a guest whose policy grants FILES_USER (place::served).

use core::cell::RefCell;

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;
use nonos_user_files::errno::{Errno, EIO};
use nonos_user_files::wire::client::{Client, Transport};

use crate::linux::abi::errno::ETIMEDOUT;

/// The broker's service, and its port when the registry has not named it.
const NAME: &[u8] = b"user_files";
const PORT: u32 = 4424;
/// How long one request may wait: a commit of a large save writes several
/// MiB to the volume before it answers.
const WAIT_MS: u64 = 30_000;

pub struct Ipc;

impl Transport for Ipc {
    fn call(&mut self, request: &[u8], reply: &mut [u8]) -> Result<usize, Errno> {
        let port = lookup_service(NAME).map_or(PORT, |p| p.port);
        let rc = mk_ipc_call_timeout(
            port as u64,
            request.as_ptr(),
            request.len(),
            reply.as_mut_ptr(),
            reply.len(),
            WAIT_MS,
        );
        match rc {
            n if n > 0 => Ok(n as usize),
            0 => Err(ETIMEDOUT as Errno),
            n => Err(i32::try_from(-n).unwrap_or(EIO)),
        }
    }
}

struct Held(RefCell<Option<Client<Ipc>>>);

// SAFETY: only the serve loop asks the broker; a guest only traps into it.
unsafe impl Sync for Held {}

static CLIENT: Held = Held(RefCell::new(None));

/// Runs `f` with this process's client to the broker.
pub fn with<T>(f: impl FnOnce(&mut Client<Ipc>) -> Result<T, Errno>) -> Result<T, Errno> {
    let mut held = CLIENT.0.try_borrow_mut().map_err(|_| EIO)?;
    f(held.get_or_insert_with(|| Client::new(Ipc)))
}
