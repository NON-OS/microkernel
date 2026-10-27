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

//! A guest's writes on a stream this capsule carries, and its close. A
//! write moves the stream on first (drive.rs); a stream net.anon opened in
//! that step is the socket's net.anon stream from then on, and is answered
//! by that path the next time the guest asks.

use super::drive::{drive, handle};
use super::entry::Phase;
use super::table;
use crate::linux::abi::errno::{self, EAGAIN, EBADF, EPIPE};
use crate::linux::guest::Guest;

/// The most one write takes from the guest.
pub const SEND_MAX: usize = super::link::TX_MAX;

pub fn send(guest: &Guest, id: u32, bytes: &[u8]) -> u64 {
    let Some(h) = handle(id) else {
        return errno::fail(EBADF);
    };
    let (asking, open) = table::with(h, |e| {
        (matches!(e.phase, Phase::Asking(_)), matches!(e.phase, Phase::Open(_)))
    })
    .unwrap_or((false, false));
    if asking {
        return super::hear::hear(guest, id, h, bytes);
    }
    /* An open stream is carried by the send itself. */
    if !open {
        drive(id, false);
    }
    let sent = table::with(h, |e| match &mut e.phase {
        Phase::Open(link) => link.send(bytes),
        Phase::Route { .. } | Phase::Anyone { .. } | Phase::Nym { .. } => Err(EAGAIN),
        Phase::Failed(errno) => Err(*errno),
        Phase::Asking(_) | Phase::Closed => Err(EPIPE),
    });
    match sent.unwrap_or(Err(EAGAIN)) {
        Ok(n) => errno::ok(n as u64),
        Err(e) => errno::fail(e),
    }
}

/// The socket is let go: its stream ends at the proxy as the entry drops.
pub fn close(h: u32) {
    drop(table::remove(h));
}
