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

//! The attestation check, which the caller supplies. The client hands an
//! `Attester` the document, this handshake's challenge and the registry
//! entries; it gets back the facts it checks the enrolment against, or a
//! refusal, which ends the session with `ABORT(POLICY)`.

/// What a document that passed says, for the client's own checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quoted {
    /// Where in the document its 32-byte challenge field starts. The
    /// client reads the challenge there itself, so an attester cannot
    /// hand back the challenge it was given in place of the document's.
    pub challenge_at: usize,
    /// The quote's P-256 attestation key, x then y.
    pub ak_public: [u8; 64],
    /// The quoted PCRs 0, 1, 2 and 7, in that order.
    pub firmware_pcrs: [[u8; 32]; 4],
}

/// Why the attester refused, by the step of nsh-v1-attest.md that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestRefusal {
    /// Step 1: the document does not parse as `NONOSATT`.
    Parse,
    /// Step 1: the document is version 4 or older.
    Version,
    /// Step 2: the document's challenge is not the one given.
    Challenge,
    /// Step 3: the quote is not a quote, its `extraData` is wrong, or the
    /// AK signature does not verify.
    Quote,
    /// Step 5: PCRs 4, 9 and 14 match no accepted release.
    ReleasePcrs,
    /// Step 6: the IOMMU is not enforcing, a grant is unconfined, or a
    /// device can still reach all of memory.
    Dma,
    /// Step 7: the registry does not fold to its root, is not complete, or
    /// its `nsh.front` entry is not the release's.
    Registry,
    /// Step 8: the policy asks for the device proof and it fails.
    DeviceProof,
    /// No attestation checker is available to this build.
    Unavailable,
}

/// The `NONOSATT` checker: steps 1, 2, 3, the release half of 5, 6, 7 and,
/// when the policy asks, 8 of nsh-v1-attest.md, against the caller's
/// policy. The client makes step 4 and the firmware half of step 5 itself
/// from what `check` returns, and checks the challenge again, read from
/// the document at `challenge_at`, never from anything echoed back.
pub trait Attester {
    /// `entries` are the registry entries from `HOST_PROOF`. A record
    /// checked offline has none, and passes `None`.
    fn check(
        &self,
        doc: &[u8],
        challenge: &[u8; 32],
        entries: Option<&[u8]>,
    ) -> Result<Quoted, AttestRefusal>;
}
