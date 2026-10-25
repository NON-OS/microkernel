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

//! Why a host was refused: at the signature over the transcript, the
//! attestation document, or the enrolment certificate.

use crate::{AttestRefusal, Half};

/// Why the host was not accepted. Each one is answered `ABORT(POLICY)`,
/// and no credential has been sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostRefusal {
    /// The host keys' signature over the transcript; the half that failed.
    HostSig(Half),
    /// The attestation checker refused the document.
    Attest(AttestRefusal),
    /// The document's challenge is not this handshake's.
    Challenge,
    /// The enrolment certificate (nsh-v1-attest.md step 4).
    Enrol(EnrolRefusal),
    /// PCRs 0, 1, 2 and 7 are not the ones the machine was enrolled with.
    FirmwarePcrs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrolRefusal {
    /// The certificate names an authority the policy does not hold.
    Authority,
    /// The authority's signature; the half that failed.
    Sig(Half),
    Fleet,
    /// It names another attestation key than the quote's.
    Ak,
    /// It names other host keys than `HOST_PROOF`'s.
    HostKeys,
    /// It names another host than the one the user typed.
    HostName,
    Expired,
    Revoked,
}
