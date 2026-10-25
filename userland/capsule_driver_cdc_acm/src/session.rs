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
//! The line taken and given up: OPEN and CLOSE, and the client that held
//! it gone (run.rs).

use nonos_libc::mk_pid_alive;

use crate::acm::{State, DTR, RTS};
use crate::control::lines;
use crate::device::Dev;
use crate::wire::E_INVAL;

/// The line taken fresh: nothing a former holder left is shown, and DTR
/// and RTS raised where the device takes them.
pub fn open(d: &mut Dev, peer: u32, body: &[u8]) -> Result<(), i32> {
    if !body.is_empty() {
        return Err(E_INVAL);
    }
    d.owner.open(peer, |p| mk_pid_alive(p))?;
    d.rx.clear();
    d.state = State::default();
    match lines(d, DTR | RTS) {
        Ok(()) | Err(E_INVAL) => Ok(()),
        Err(e) => {
            d.owner.close();
            Err(e)
        }
    }
}

/// DTR and RTS dropped (a modem hangs up), the bytes left forgotten.
pub fn close(d: &mut Dev) {
    let _ = lines(d, 0);
    d.rx.clear();
    d.owner.close();
}
