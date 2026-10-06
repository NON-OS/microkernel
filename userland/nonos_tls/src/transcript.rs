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

//! The handshake transcript, kept as a running hash.

use sha2::{Digest, Sha256};

/*
 * RFC 8446 section 4.4.1 only ever asks for the hash of the transcript up to
 * some message. The bytes themselves were kept and re-hashed whole for the
 * handshake keys, for CertificateVerify (after cloning them), for Finished and
 * for the application keys. A running state answers each of those by copying
 * a hundred bytes of hash state, however long the certificate chain was.
 */
#[derive(Clone, Default)]
pub struct Transcript {
    hash: Sha256,
}

impl Transcript {
    pub fn new() -> Self {
        Self { hash: Sha256::new() }
    }

    /// Append one whole handshake message.
    pub fn push(&mut self, message: &[u8]) {
        self.hash.update(message);
    }

    /// The hash of everything pushed so far; the running state is kept.
    pub fn digest(&self) -> [u8; 32] {
        let mut out = [0u8; 32];
        out.copy_from_slice(&self.hash.clone().finalize());
        out
    }
}
