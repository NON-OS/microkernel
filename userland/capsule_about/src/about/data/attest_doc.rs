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

//! The signed statement the machine hands to whoever asks what it is running.
//!
//! This is the strongest evidence on the Verify screen and the only one whose
//! authority does not rest on trusting the software presenting it: the TPM signs
//! a root over the capsule registry together with the challenge it was given, so
//! a document cannot be replayed and cannot be assembled by a compromised
//! userland that has no access to the key.
//!
//! Asking costs a TPM signature, so it is asked once, when the user opens the
//! screen, and never on a repaint.
//!
//! A machine without a TPM refuses, and that refusal is reported as a refusal.
//! It is not a failure of the system and it is not a pass either; a verification
//! surface that quietly showed nothing here would be the worst of the three.

use nonos_libc::mk_attest_doc;

use super::attest_challenge::challenge;
use super::doc_parse::{parse, Doc, DOC_CAP};

pub enum Attestation {
    /// The screen has not been opened yet. Nothing has been asked of the TPM.
    NotAsked,
    /// The machine declined. No reason is carried, because there is none to
    /// carry: the kernel deliberately returns one value for "no TPM" and for "the
    /// registry is incomplete" so a caller cannot probe the machine's state
    /// through the failure path. Storing the errno would only invite a later
    /// reader to decode what the kernel took care not to say.
    Refused,
    /// A document came back but did not parse as one. Reported separately from a
    /// refusal because it means something is wrong rather than absent.
    Malformed,
    Produced(Doc),
}

/// Ask the machine to attest itself. Called once per screen open.
pub fn request() -> Attestation {
    let challenge = challenge();
    let mut buf = [0u8; DOC_CAP];
    let rc = mk_attest_doc(&challenge, &mut buf);
    if rc < 0 {
        return Attestation::Refused;
    }
    let len = rc as usize;
    if len > buf.len() {
        return Attestation::Malformed;
    }
    match parse(&buf[..len], &challenge) {
        Some(doc) => Attestation::Produced(doc),
        None => Attestation::Malformed,
    }
}
