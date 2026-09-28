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

//! A relay as the two halves of the directory describe it.

use crate::circuit::NextHop;

/// The flags a consensus `s` line can carry that this capsule acts on.
#[derive(Clone, Copy, Default)]
pub struct Flags {
    pub running: bool,
    pub valid: bool,
    pub fast: bool,
    pub stable: bool,
    pub guard: bool,
    pub exit: bool,
    pub authority: bool,
}

/// One relay, assembled from a consensus entry and the microdescriptor it
/// points at. Both halves are required: the consensus says where a relay is
#[derive(Clone)]
pub struct Relay {
    pub address: [u8; 4],
    pub or_port: u16,
    /// SHA-1 of the RSA identity key, which is the name a handshake uses.
    pub rsa_identity: [u8; 20],
    pub ed25519_identity: [u8; 32],
    pub ntor_onion_key: [u8; 32],
    pub flags: Flags,
    /// The consensus weight, in the units a `w Bandwidth=` line uses.
    pub weight: u32,
    /// True when the relay's exit policy accepts port 80 or 443.
    pub exits_web: bool,
}

impl Relay {
    pub fn next_hop(&self) -> NextHop {
        NextHop {
            address: self.address,
            port: self.or_port,
            rsa_identity: self.rsa_identity,
            ed25519_identity: self.ed25519_identity,
        }
    }

    /// Usable at all: running, valid, and carrying the keys a circuit needs.
    /// An entry that is missing its microdescriptor half arrives with a zero
    pub fn usable(&self) -> bool {
        self.flags.running
            && self.flags.valid
            && self.or_port != 0
            && self.ntor_onion_key.iter().any(|b| *b != 0)
    }
}
