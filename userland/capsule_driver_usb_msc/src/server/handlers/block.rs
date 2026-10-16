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

//! The block surface, for the kernel's client alone (server/admits.rs):
//! capacity, whole 512-byte sectors in and out on a device of any served
//! block length, and a flush, so the medium is written only through the
//! kernel's block layer.

use super::block_ops::serve;
use crate::protocol::{Request, E_AGAIN, E_IO, E_NODEV, E_TIMEDOUT, OP_BLK_READ, OP_BLK_WRITE};
use crate::scan::Medium;
use crate::server::respond;
use crate::state::{Said, State};

pub fn handle(
    state: &mut State,
    medium: &mut Medium,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) {
    let status = match medium {
        Medium::Ready(disk) => serve(state, disk, req, body, tx),
        Medium::Scanning | Medium::Checking(_) => Err(E_AGAIN),
        Medium::Absent | Medium::Changed => Err(E_NODEV),
    };
    // A device that failed a request is asked whether it is still there,
    // and whether its medium changed, before it serves another
    // (scan/check.rs). From the first sector read or written the kernel
    // holds something of the medium, and a change ends serving. Before it,
    // one said under the request is served afresh, and the kernel is asked
    // to come again: its disk selection passed the stick over on E_IO.
    let status = match (status, &*medium) {
        (Err(e @ (E_IO | E_TIMEDOUT)), Medium::Ready(disk)) => {
            *medium = Medium::Checking(*disk);
            let said = state.said() > Said::Nothing && !state.held();
            Err(if e == E_IO && said { E_AGAIN } else { e })
        }
        (Ok(n), _) if matches!(req.op, OP_BLK_READ | OP_BLK_WRITE) => {
            state.hold();
            Ok(n)
        }
        (status, _) => status,
    };
    // A write or a flush replies with no data: a payload of none is the
    // bare status word.
    let _ = match status {
        Ok(n) => respond::payload(sender, req, n, tx),
        Err(e) => respond::status(sender, req, e, tx),
    };
}
