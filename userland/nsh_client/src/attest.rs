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

pub use crate::quoted::Quoted;
use nsh_wire::Enrol;

/// Why the attester refused, by the step of docs/nsh/nsh-v1-attest.md that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestRefusal {
    /// Step 1: the document does not parse as `NONOSATT`.
    Parse,
    /// Step 1: the document is a version other than 5.
    Version,
    /// Step 2: the document's challenge is not the one given.
    Challenge,
    /// Step 3: the quote is not a quote, its `extraData` is wrong, or the
    /// AK signature does not verify.
    Quote,
    /// Step 5: PCRs 8 and 9 match no accepted release.
    ReleasePcrs,
    /// Step 5: the event log does not parse.
    Log,
    /// Step 5: the log does not replay to a quoted PCR.
    Replay,
    /// Step 5: PCR 4 holds an application that is neither the release's
    /// loader nor one the enrolment lists.
    BootApplication,
    /// Step 5: the release's loader is not among PCR 4's applications.
    Loader,
    /// Step 5: PCR 14 records a capsule admitted on its path alone, as a
    /// development image admits every capsule.
    PathOnly,
    /// Step 5: PCR 14 records no capsule, or one a person's root or a
    /// local build admitted rather than the vendor's proof.
    NotVendor,
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
/// when the policy asks, 8 of docs/nsh/nsh-v1-attest.md, against the caller's
/// policy. The client makes step 4 and the firmware half of step 5 itself
/// from what `check` returns, and checks the challenge again, read from
/// the document at `challenge_at`, never from anything echoed back.
pub trait Attester {
    /// `entries` are the registry entries from `HOST_PROOF`. A record
    /// checked offline has none, and passes `None`. `enrol` is the host's
    /// enrolment as sent, not yet checked: its boot applications are the
    /// ones PCR 4 may hold besides the loader, and the client checks its
    /// signature and every other field itself once the attester passes.
    fn check(
        &self,
        doc: &[u8],
        challenge: &[u8; 32],
        entries: Option<&[u8]>,
        enrol: &Enrol<'_>,
    ) -> Result<Quoted, AttestRefusal>;
}
