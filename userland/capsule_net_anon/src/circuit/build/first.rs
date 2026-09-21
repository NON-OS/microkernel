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

//! The first hop, which is a bare CREATE2 with no onion layers yet.

use crate::link::Link;
use crate::ntor::Handshake;
use crate::path::Relay;
use crate::trace;

use super::super::create::{create2, created2_reply};
use super::super::hop::Hop;
use super::error::BuildError;
use super::reply::created2;

pub(super) fn first(link: &mut Link, circuit: u32, guard: &Relay) -> Result<Hop, BuildError> {
    let handshake = Handshake::new(guard.rsa_identity, guard.ntor_onion_key)
        .map_err(|_| BuildError::Handshake)?;
    let cell = create2(circuit, &handshake.onionskin()).ok_or(BuildError::Protocol)?;
    link.send(&cell.encode()).map_err(|_| BuildError::Link)?;

    let answer = created2(link, circuit)?;
    let reply = created2_reply(&answer.payload).ok_or(BuildError::Protocol)?;
    let keys = handshake.finish(reply).map_err(|_| BuildError::Handshake)?;
    trace::say(b"circuit hop 1 up");
    Ok(Hop::new(&keys))
}
