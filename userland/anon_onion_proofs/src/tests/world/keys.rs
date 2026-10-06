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


//! Fixed key material for the simulated network, from a label, so every run
//! is the same. Throwaway test seeds: they sign and decrypt nothing real.

use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};

/// 32 bytes named by `label` and `index`.
pub fn bytes32(label: &str, index: u32) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(label.as_bytes());
    h.update(index.to_be_bytes());
    h.finalize().into()
}

/// An X25519 secret and its public half.
pub fn x25519(label: &str, index: u32) -> (StaticSecret, [u8; 32]) {
    let secret = StaticSecret::from(bytes32(label, index));
    let public = PublicKey::from(&secret).to_bytes();
    (secret, public)
}

/// A running source of 32 byte values for ephemeral keys.
pub struct Draw {
    label: &'static str,
    next: u32,
}

impl Draw {
    pub fn new(label: &'static str) -> Self {
        Self { label, next: 0 }
    }

    pub fn x25519(&mut self) -> (StaticSecret, [u8; 32]) {
        self.next += 1;
        x25519(self.label, self.next)
    }
}
