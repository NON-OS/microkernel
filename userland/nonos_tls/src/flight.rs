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

use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct ClientFlight {
    pub record: Vec<u8>,
    pub handshake: Vec<u8>,
    /// The X25519 private key. Boxed, as is the scalar below: a flight is
    /// returned and moved by value, and a move copies only the pointer.
    pub private: Box<[u8; 32]>,
    /// The secp256r1 private scalar, for a server that takes the P-256 share.
    pub p256_private: Box<[u8; 32]>,
}

impl ClientFlight {
    /// The two private keys, which with the transcript recover every traffic
    /// key, wiped where they lie: for a caller that keeps the flight past the
    /// handshake (a connection's `settle`).
    pub fn wipe_private(&mut self) {
        super::traffic_keys::wipe(&mut self.private[..]);
        super::traffic_keys::wipe(&mut self.p256_private[..]);
    }
}

impl Drop for ClientFlight {
    fn drop(&mut self) {
        self.wipe_private();
    }
}
