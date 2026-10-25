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
//! A request sent, cancelled, or let go. One let go while it ran is
//! cancelled on its channel and the channel counted spent, so the next
//! request goes on a new one and the late answer cannot pass for its own.

use nonos_libc::Deadline;

use crate::ctaphid::{send, Job, CMD_CANCEL, CMD_CBOR};
use crate::device::{Dev, QUIET_MS};
use crate::up::channel;
use crate::wire::{E_BUSY, E_INVAL, E_IO, E_NODEV};

pub fn start(d: &mut Dev, cbor: &[u8]) -> Result<(), i32> {
    if d.job.busy() {
        return Err(E_BUSY);
    }
    let (Some(mut bus), Some(dci)) = (d.bus(), d.port().map(|p| p.dci)) else {
        return Err(E_NODEV);
    };
    if d.spent {
        let ch = channel(&mut bus, dci).map_err(|_| E_IO)?;
        if let Some(b) = d.bound.as_mut() {
            b.nic.ch = ch;
        }
        d.spent = false;
    }
    let cid = d.port().map_or([0; 4], |p| p.ch.cid);
    send(&mut bus, dci.1, cid, CMD_CBOR, cbor).map_err(|_| E_IO)?;
    d.job = Job::start(cid);
    d.quiet = Deadline::after_ms(QUIET_MS);
    Ok(())
}

/// CTAPHID_CANCEL: the key ends the request with its own error status.
pub fn cancel(d: &mut Dev) -> Result<(), i32> {
    if !d.job.busy() {
        return Err(E_INVAL);
    }
    send_cancel(d)
}

/// The holder gone, or the key quiet: `ran` says a request was running.
pub fn spent(d: &mut Dev, ran: bool) {
    if ran {
        let _ = send_cancel(d);
        d.spent = true;
    }
}

fn send_cancel(d: &mut Dev) -> Result<(), i32> {
    let (Some(mut bus), Some(p)) = (d.bus(), d.port()) else { return Err(E_NODEV) };
    let (out, cid) = (p.dci.1, p.ch.cid);
    send(&mut bus, out, cid, CMD_CANCEL, &[]).map_err(|_| E_IO)
}
