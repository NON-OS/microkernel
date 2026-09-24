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

//! Reading to NETINFO, verifying CERTS on the way.

use crate::cell::{Frame, CELL_AUTH_CHALLENGE, CELL_CERTS, CELL_NETINFO, CELL_VPADDING};
use crate::path::Relay;
use crate::trace;

use super::bind::bind;
use super::netinfo::ours;
use super::session::{Link, LinkError};
use super::timing::HANDSHAKE_MS;

pub(super) fn drain(
    link: &mut Link,
    leaf: &[u8],
    relay: &Relay,
    now: u64,
) -> Result<(), LinkError> {
    let mut bound = false;
    loop {
        let Some(Frame::Var(cell)) = link.recv(HANDSHAKE_MS)? else {
            return Err(LinkError::Protocol);
        };
        match cell.command {
            CELL_CERTS => {
                bind(&cell.body, leaf, &relay.ed25519_identity, now).map_err(|_| {
                    trace::say(b"link certs rejected");
                    LinkError::Identity
                })?;
                bound = true;
                trace::say(b"link identity proved");
            }
            /*
             * A relay offers AUTH_CHALLENGE so a peer can prove it is also a
             * relay. A client does not authenticate, so there is nothing to say.
             */
            CELL_AUTH_CHALLENGE | CELL_VPADDING => {}
            CELL_NETINFO if bound => {
                let netinfo = ours(relay.address);
                link.send(&netinfo.encode())?;
                return Ok(());
            }
            /*
             * NETINFO before a verified CERTS would finish the handshake with a
             * peer that never proved who it is, so it is refused here rather
             * than accepted and checked afterwards.
             */
            _ => return Err(LinkError::Protocol),
        }
    }
}
