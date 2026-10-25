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

//! The handshake's second half: the gateway's material verified under the
//! derived key, ours sealed and sent, and the gateway's answer read.

use super::frame::{decode, encode, HandshakeFrameError, PAYLOAD_TAG};
use super::seal_material::seal_material;
use super::verify_material::verify_material;
use super::wire::{HandshakeError, Identity, Wire};

pub(super) fn finish<W: Wire>(
    wire: &mut W,
    identity: &Identity<'_>,
    version: u64,
    eph: &super::ephemeral::Ephemeral,
    gateway_ephemeral: &[u8; super::sizes::EPHEMERAL_BYTES],
    gateway_material: &super::material::Material,
    shared: &[u8; 32],
) -> Result<(), HandshakeError> {
    if !verify_material(
        identity.gateway_public,
        gateway_ephemeral,
        &eph.public,
        shared,
        gateway_material,
    ) {
        return Err(HandshakeError::BadSignature);
    }

    let ours = seal_material(identity.own_seed, &eph.public, gateway_ephemeral, shared)
        .map_err(|_| HandshakeError::Crypto)?;
    wire.send_text(&encode(PAYLOAD_TAG, version, &ours.to_bytes()))?;

    match decode(&wire.recv_text()?).map_err(map_frame)?.as_slice() {
        [1] => Ok(()),
        _ => Err(HandshakeError::Refused),
    }
}

pub(super) fn map_frame(e: HandshakeFrameError) -> HandshakeError {
    match e {
        HandshakeFrameError::Refused => HandshakeError::Refused,
        HandshakeFrameError::Malformed => HandshakeError::Malformed,
    }
}
