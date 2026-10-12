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
//! The key taken and given up: OPEN and CLOSE, and the client that held
//! it gone (run.rs). A request or answer is never handed from one holder
//! to the next.

use nonos_libc::mk_pid_alive;

use crate::device::Dev;
use crate::request::spent;
use crate::wire::E_INVAL;

pub fn open(d: &mut Dev, peer: u32, body: &[u8]) -> Result<(), i32> {
    if !body.is_empty() {
        return Err(E_INVAL);
    }
    let was = d.owner.pid;
    d.owner.open(peer, |p| mk_pid_alive(p))?;
    // A holder that died and was not yet let go leaves its request behind.
    if was != Some(peer) {
        let ran = d.job.abandon();
        spent(d, ran);
    }
    Ok(())
}

pub fn close(d: &mut Dev) {
    let ran = d.job.abandon();
    spent(d, ran);
    d.owner.close();
}
