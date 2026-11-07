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

//! The kernel's attestation: the spec's document over each challenge the
//! server asks for, or what a proof puts in its place.

use super::attest_build::keys::{hybrid, HOST};
use super::attest_build::Spec;
use super::host::spec;
use nsh_server::{AttestFault, Attestation};

/// The kernel's attestation: the spec's document over each challenge.
pub struct Kernel {
    pub fault: Option<AttestFault>,
    /// Entries in place of the spec's.
    pub entries: Option<Vec<u8>>,
    /// The challenges the kernel was asked to quote.
    pub asked: Vec<[u8; 32]>,
    /// Quote a challenge of another handshake in place of the one asked.
    pub stale: bool,
    /// A document in place of the kernel's.
    pub doc: Option<Vec<u8>>,
}

impl Kernel {
    pub fn honest() -> Self {
        Kernel { fault: None, entries: None, asked: Vec::new(), stale: false, doc: None }
    }
}

impl Attestation for Kernel {
    fn document(&mut self, challenge: &[u8; 32]) -> Result<Vec<u8>, AttestFault> {
        self.asked.push(*challenge);
        match self.fault {
            Some(f) => Err(f),
            None if self.doc.is_some() => Ok(self.doc.clone().unwrap_or_default()),
            None if self.stale => Ok(Spec { challenge: Some([0x99; 32]), ..spec() }.doc_bytes()),
            None => Ok(Spec { challenge: Some(*challenge), ..spec() }.doc_bytes()),
        }
    }
    fn entries(&mut self) -> Result<Vec<u8>, AttestFault> {
        Ok(self.entries.clone().unwrap_or_else(|| spec().entries.concat()))
    }
}

/// The document the kernel makes for the handshake `th`, as it sent it.
pub fn doc_for(th: &[u8; 64]) -> Vec<u8> {
    let host = hybrid(HOST);
    let challenge = nsh_crypto::attest_challenge(th, &host.ed.public, &host.ml_public[..]);
    Spec { challenge: Some(challenge), ..spec() }.doc_bytes()
}
