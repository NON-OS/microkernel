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

//! Asking for the next hop: CREATE2 to the guard, EXTEND2 through the rest.

use crate::cell::{pack, Cell, RelayHeader, CELL_RELAY_EARLY, RELAY_EXTEND2};
use crate::link::Link;
use crate::ntor::Handshake;
use crate::path::Relay;

use super::super::create::create2;
use super::super::extend::extend2_body;
use super::super::hop::Hop;
use super::super::seal::seal;
use super::error::BuildError;

/// Send CREATE2 to `guard` and return the handshake its CREATED2 answers.
pub fn ask_create(link: &mut Link, circuit: u32, guard: &Relay) -> Result<Handshake, BuildError> {
    let handshake = Handshake::new(guard.rsa_identity, guard.ntor_onion_key)
        .map_err(|_| BuildError::Handshake)?;
    let cell = create2(circuit, &handshake.onionskin()).ok_or(BuildError::Protocol)?;
    link.send(&cell.encode()).map_err(|_| BuildError::Link)?;
    Ok(handshake)
}

/// Ask the last hop in `hops` to extend the circuit to `next`, and return
/// the handshake its EXTENDED2 answers.
pub fn ask_extend(
    link: &mut Link,
    circuit: u32,
    hops: &mut [Hop],
    next: &Relay,
) -> Result<Handshake, BuildError> {
    let target = hops.len().checked_sub(1).ok_or(BuildError::Protocol)?;
    let handshake = Handshake::new(next.rsa_identity, next.ntor_onion_key)
        .map_err(|_| BuildError::Handshake)?;
    let body = extend2_body(&next.next_hop(), &handshake.onionskin());
    let header =
        RelayHeader { command: RELAY_EXTEND2, recognized: 0, stream: 0, length: body.len() as u16 };
    let mut payload = pack(&header, &body).ok_or(BuildError::Protocol)?;
    seal(hops, target, &mut payload).ok_or(BuildError::Protocol)?;
    /* RELAY_EARLY, as tor-spec 5.6 requires for EXTEND2. */
    let mut cell = Cell::new(circuit, CELL_RELAY_EARLY);
    cell.payload = payload;
    link.send(&cell.encode()).map_err(|_| BuildError::Link)?;
    Ok(handshake)
}
