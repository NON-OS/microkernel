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

//! Finishing a hop from the reply that answers its handshake.

use crate::cell::{Cell, CELL_CREATED2, CELL_DESTROY, CELL_RELAY, CELL_RELAY_EARLY};
use crate::ntor::KEY_MATERIAL_BYTES;

use super::super::create::created2_reply;
use super::super::extend::extended2_reply;
use super::super::hop::Hop;
use super::super::open::open;
use super::error::BuildError;
use super::reply::extended2_body;

/// The hop that `cell` brings up, given the hops already up. With none up
/// the answer is a cleartext CREATED2; after that it is an EXTENDED2 that
/// only the last hop may have sent, checked against its digest. `finish`
/// completes the pending handshake from the relay's reply.
pub fn answer(
    cell: &mut Cell,
    hops: &mut [Hop],
    finish: impl FnOnce(&[u8]) -> Option<[u8; KEY_MATERIAL_BYTES]>,
) -> Result<Hop, BuildError> {
    if cell.command == CELL_DESTROY {
        return Err(BuildError::Destroyed);
    }
    let reply = match hops.len().checked_sub(1) {
        None if cell.command == CELL_CREATED2 => created2_reply(&cell.payload),
        Some(target) if cell.command == CELL_RELAY || cell.command == CELL_RELAY_EARLY => {
            let opened = open(hops, &mut cell.payload).ok_or(BuildError::Unrecognised)?;
            if opened.hop != target {
                return Err(BuildError::Protocol);
            }
            extended2_reply(extended2_body(&cell.payload)?)
        }
        _ => return Err(BuildError::Protocol),
    };
    let keys = finish(reply.ok_or(BuildError::Protocol)?).ok_or(BuildError::Handshake)?;
    Ok(Hop::new(&keys))
}
