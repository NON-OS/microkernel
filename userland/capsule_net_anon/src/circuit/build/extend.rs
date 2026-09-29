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

//! Extending an existing circuit by one hop.

extern crate alloc;

use crate::cell::{pack, Cell, RelayHeader, CELL_RELAY_EARLY, RELAY_EXTEND2};
use crate::link::Link;
use crate::ntor::Handshake;
use crate::path::Relay;
use crate::trace;

use super::super::extend::{extend2_body, extended2_reply};
use super::super::hop::Hop;
use super::super::open::open;
use super::super::seal::seal;
use super::error::BuildError;
use super::reply::{extended2, extended2_body};

/// Ask the last hop to extend the circuit to `next`, and return the new hop.
pub(super) fn extend(
    link: &mut Link,
    circuit: u32,
    hops: &mut alloc::vec::Vec<Hop>,
    next: &Relay,
) -> Result<Hop, BuildError> {
    let target = hops.len().checked_sub(1).ok_or(BuildError::Protocol)?;
    let handshake = Handshake::new(next.rsa_identity, next.ntor_onion_key)
        .map_err(|_| BuildError::Handshake)?;
    let body = extend2_body(&next.next_hop(), &handshake.onionskin());
    let header =
        RelayHeader { command: RELAY_EXTEND2, recognized: 0, stream: 0, length: body.len() as u16 };
    let mut payload = pack(&header, &body).ok_or(BuildError::Protocol)?;
    seal(hops, target, &mut payload).ok_or(BuildError::Protocol)?;

    let mut cell = Cell::new(circuit, CELL_RELAY_EARLY);
    cell.payload = payload;
    link.send(&cell.encode()).map_err(|_| BuildError::Link)?;

    let mut answer = extended2(link, circuit)?;
    let opened = open(hops, &mut answer.payload).ok_or(BuildError::Unrecognised)?;
    if opened.hop != target {
        return Err(BuildError::Protocol);
    }
    let body = extended2_body(&answer.payload)?;
    let reply = extended2_reply(body).ok_or(BuildError::Protocol)?;
    let keys = handshake.finish(reply).map_err(|_| BuildError::Handshake)?;
    trace::say_num(b"circuit hop up", hops.len() as u64 + 1);
    Ok(Hop::new(&keys))
}
