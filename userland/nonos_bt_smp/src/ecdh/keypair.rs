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

//! This host's key pair for one pairing. The secret comes from the
//! caller's random source and is used for one pairing only, as Vol 3,
//! Part H, 2.3.5.6.1 asks; nonos_p256 overwrites it when the pair is
//! dropped.

use super::peer::PeerKey;
use crate::error::SmpError;
use nonos_p256::Secret;

pub struct KeyPair {
    secret: Secret,
    x: [u8; 32],
    y: [u8; 32],
}

impl KeyPair {
    /// The pair for a 32-octet secret, most significant octet first;
    /// `BadSecret` if it is zero or not below the group order, so the
    /// caller draws again.
    pub fn from_secret(secret: [u8; 32]) -> Result<Self, SmpError> {
        let secret = Secret::from_bytes(&secret).ok_or(SmpError::BadSecret)?;
        let xy = secret.public_xy().ok_or(SmpError::BadSecret)?;
        let (mut x, mut y) = ([0u8; 32], [0u8; 32]);
        x.copy_from_slice(&xy[..32]);
        y.copy_from_slice(&xy[32..]);
        Ok(Self { secret, x, y })
    }

    /// The public key's coordinates, most significant octet first.
    pub fn public(&self) -> (&[u8; 32], &[u8; 32]) {
        (&self.x, &self.y)
    }

    /// The DHKey: the x coordinate of the secret times the peer's point,
    /// most significant octet first (Vol 3, Part H, 2.3.5.6.1).
    pub fn dh_key(&self, peer: &PeerKey) -> Result<[u8; 32], SmpError> {
        self.secret.shared_x(peer.point()).ok_or(SmpError::BadKey)
    }
}
