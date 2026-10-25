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

//! Reading a `HOST_PROOF` and verifying its host signatures: Ed25519 and
//! ML-DSA-65, both, over `H("nsh/1 host-sig\0" || TH(HOST_PROOF without
//! its signatures))` (nsh-v1-attest.md).

use nonos_ed25519::Signature;
use nsh_crypto::{labelled, Label, Transcript};
use nsh_wire::{HostProof, HybridPublic, Kind, Reader, WireError};
use nsh_wire::{ENTRY_LEN, MAX_DOC, MAX_ENROL, MAX_ENTRIES};

use crate::Evidence;

/// Why a `HOST_PROOF` gave no evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceError {
    /// nsh_wire refused the body.
    Wire(WireError),
    /// Either half of the host signature fails.
    Signature,
}

impl<'a> Evidence<'a> {
    /// The parts of a `HOST_PROOF` body, once both host signatures verify
    /// over the transcript as it stands before this frame is pushed.
    pub fn verify(body: &'a [u8], transcript: &Transcript) -> Result<Self, EvidenceError> {
        let proof = HostProof::parse(body).map_err(EvidenceError::Wire)?;
        let th = transcript.without_sigs(Kind::HostProof, proof.signed);
        let digest = labelled(Label::HostSig, &th);
        let ed =
            nonos_ed25519::verify(proof.host.ed, &digest, &Signature::from_bytes(proof.sig.ed));
        let ml = nonos_mldsa65::verify(proof.host.ml, &digest, proof.sig.ml);
        if !(ed && ml) {
            return Err(EvidenceError::Signature);
        }
        parts(body).map_err(EvidenceError::Wire)
    }
}

fn parts(body: &[u8]) -> Result<Evidence<'_>, WireError> {
    let mut r = Reader::new(body);
    let host = HybridPublic::read(&mut r)?;
    let enrol = r.bytes16(1, MAX_ENROL)?;
    let doc = r.bytes32(1, MAX_DOC)?;
    let entries = r.bytes32(ENTRY_LEN, ENTRY_LEN * MAX_ENTRIES)?;
    Ok(Evidence { host_ed: host.ed, host_ml: host.ml, enrol, doc, entries })
}
