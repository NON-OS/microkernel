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

//! `HOST_PROOF`: the host keys, the enrolment certificate, the kernel's
//! attestation document over this handshake's challenge, the registry
//! entries, and the host keys' signature over the transcript. Every field
//! is held to the client's rules before it is laid out: the enrolment when
//! the config is made, the document and entries here, so the server never
//! sends a proof the client would refuse as broken.

use super::Server;
use crate::{AttestFault, Attestation, HostSigner, Rand, Roles, Why};
use nsh_crypto::{attest_challenge, labelled, Label};
use nsh_wire::keys::{HYBRID_PUBLIC_LEN, HYBRID_SIG_LEN};
use nsh_wire::{Expect, Kind, Writer, ENTRY_LEN, MAX_DOC, MAX_ENTRIES};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn host_proof(&mut self, rand: Rand<'_>) -> Result<(), Why> {
        let (ed, ml) = (self.signer.ed_public(), self.signer.ml_public());
        let challenge = attest_challenge(&self.th_hello, ed, ml);
        let doc = self.attest.document(&challenge).map_err(Why::Attest)?;
        let entries = self.attest.entries().map_err(Why::Attest)?;
        let whole = entries.len() % ENTRY_LEN == 0;
        let sized = (ENTRY_LEN..=ENTRY_LEN * MAX_ENTRIES).contains(&entries.len());
        if doc.is_empty() || doc.len() > MAX_DOC || !whole || !sized {
            return Err(Why::Attest(AttestFault::Malformed));
        }
        let enrol = &self.cfg.enrol;
        let n = HYBRID_PUBLIC_LEN + 2 + enrol.len() + 4 + doc.len() + 4 + entries.len();
        let mut body = alloc::vec![0u8; n];
        let mut w = Writer::new(&mut body);
        w.put(ed).and_then(|_| w.put(ml)).map_err(Why::Wire)?;
        w.bytes16(enrol).map_err(Why::Wire)?;
        w.bytes32(&doc).map_err(Why::Wire)?;
        w.bytes32(&entries).map_err(Why::Wire)?;
        let digest = labelled(Label::HostSig, &self.th.without_sigs(Kind::HostProof, &body));
        let sig = self.signer.sign(&digest, rand).ok_or(Why::Sign)?;
        body.reserve_exact(HYBRID_SIG_LEN);
        body.extend_from_slice(&sig.ed);
        body.extend_from_slice(&sig.ml[..]);
        self.send(Kind::HostProof, &body, Expect::HostProof)?;
        self.th.push(Kind::HostProof, &body);
        self.doc_hash = nonos_hash::sha512(&doc);
        Ok(())
    }
}
