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

use super::derive::derive_shared_key;
use super::ephemeral::new_ephemeral;
use super::finish::{finish, map_frame};
use super::frame::{decode, encode, INIT_TAG};
use super::init::init_message;
use super::parse_gateway::parse_gateway_material;
use super::wire::{HandshakeError, Identity, Wire};

/// Run the three-message registration and return the gateway shared key.
pub fn run_handshake<W: Wire>(
    wire: &mut W,
    identity: &Identity<'_>,
    version: u64,
) -> Result<[u8; 32], HandshakeError> {
    let eph = new_ephemeral()?;
    let init = init_message(identity.own_public, &eph.public, &eph.salt);
    wire.send_text(&encode(INIT_TAG, version, &init))?;

    let payload = decode(&wire.recv_text()?).map_err(map_frame)?;
    let (gateway_ephemeral, gateway_material) =
        parse_gateway_material(&payload).ok_or(HandshakeError::Malformed)?;

    // The salt is ours, so the gateway cannot steer the derivation.
    let mut shared = derive_shared_key(&eph.secret, &gateway_ephemeral, &eph.salt)
        .map_err(|_| HandshakeError::Crypto)?;
    let outcome =
        finish(wire, identity, version, &eph, &gateway_ephemeral, &gateway_material, &shared);
    // The caller keeps the key only when the gateway took it; every other end
    // leaves no copy here.
    if let Err(e) = outcome {
        crate::crypto::wipe::wipe(&mut shared);
        return Err(e);
    }
    Ok(shared)
}
