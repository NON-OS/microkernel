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

//! X25519 on the host, as shield_core pins it.

use nonos_nox_chunks::X25519;
use x25519_dalek::{PublicKey, StaticSecret};

pub struct D;
impl X25519 for D {
    fn dh(&self, sk: &[u8; 32], pk: &[u8; 32]) -> [u8; 32] {
        StaticSecret::from(*sk).diffie_hellman(&PublicKey::from(*pk)).to_bytes()
    }
    fn public(&self, sk: &[u8; 32]) -> [u8; 32] {
        PublicKey::from(&StaticSecret::from(*sk)).to_bytes()
    }
}
