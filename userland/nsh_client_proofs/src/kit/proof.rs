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

//! `HOST_PROOF` as the test server builds it: the host keys, an enrolment,
//! a document for this handshake's challenge, registry entries, and the
//! hybrid signature over the transcript. A proof changes one field.

use super::attest::doc;
use super::enrol::{signed, EnrolSpec, SigMode};
use super::keys::{self, Hybrid};
use super::server::Server;
use super::{put16, put32};
use nsh_crypto::{attest_challenge, labelled, Label};
use nsh_wire::Kind;

#[derive(Clone)]
pub struct ProofSpec {
    /// The keys `HOST_PROOF` names.
    pub host: &'static Hybrid,
    /// The key that signs it, and how.
    pub signer: &'static Hybrid,
    pub sig: SigMode,
    pub enrol: EnrolSpec,
    /// The document; `None` is the good one for this handshake.
    pub doc: Option<Vec<u8>>,
    pub entries: Vec<u8>,
}

impl ProofSpec {
    pub fn good() -> Self {
        let h = keys::host();
        ProofSpec {
            host: h,
            signer: h,
            sig: SigMode::Good,
            enrol: EnrolSpec::good(),
            doc: None,
            entries: vec![7; 90],
        }
    }

    pub fn challenge(&self, s: &Server) -> [u8; 32] {
        attest_challenge(&s.th_hello, &self.host.ed.public, &self.host.ml_public)
    }

    /// The body for the server's transcript as it stands.
    pub fn bytes(&self, s: &Server) -> Vec<u8> {
        let mut body = self.host.public_bytes();
        put16(&mut body, &self.enrol.bytes());
        put32(&mut body, &self.doc.clone().unwrap_or_else(|| doc(&self.challenge(s))));
        put32(&mut body, &self.entries);
        let digest = labelled(Label::HostSig, &s.th.without_sigs(Kind::HostProof, &body));
        body.extend_from_slice(&signed(self.signer, &digest, self.sig));
        body
    }
}
