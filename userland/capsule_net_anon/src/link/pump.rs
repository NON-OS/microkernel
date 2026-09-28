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

use crate::cell::Frame;
use crate::path::Relay;
use crate::trace;

use super::bind::bind;
use super::netinfo::ours;
use super::session::{Link, LinkError};
use super::step::{classify, Next};
use super::timing::HANDSHAKE_MS;

pub(super) fn drain(
    link: &mut Link,
    leaf: &[u8],
    relay: &Relay,
    now: u64,
) -> Result<(), LinkError> {
    let mut bound = false;
    loop {
        // A relay that says nothing within the handshake budget has not
        // finished the handshake; that is a failed link, named as such.
        let Some(frame) = link.recv(HANDSHAKE_MS)? else {
            trace::say(b"link handshake timed out");
            return Err(LinkError::Protocol);
        };
        let (variable, command) = match &frame {
            Frame::Var(cell) => (true, cell.command),
            Frame::Fixed(cell) => (false, cell.command),
        };
        match classify(variable, command, bound) {
            Next::Certs => {
                let Frame::Var(cell) = frame else { return Err(LinkError::Protocol) };
                bind(&cell.body, leaf, &relay.ed25519_identity, now).map_err(|_| {
                    trace::say(b"link certs rejected");
                    LinkError::Identity
                })?;
                bound = true;
                trace::say(b"link identity proved");
            }
            Next::Ignore => {}
            Next::Finish => {
                let netinfo = ours(relay.address);
                link.send(&netinfo.encode())?;
                trace::say(b"link open");
                return Ok(());
            }
            Next::Refuse => return Err(LinkError::Protocol),
        }
    }
}
