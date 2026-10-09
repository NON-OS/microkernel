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

//! Why a statement was not assembled. Each variant names the one invariant
//! that failed; nothing is proven after any of them.

use nonos_device_attest::RegistryError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The request is shorter than its fixed fields, longer than
    /// `REQUEST_MAX`, or not exactly its fields and the verifier id.
    RequestSize,
    /// The request does not start with `NZKDREQ1`.
    RequestMagic,
    /// A word of the context nonce is p or above.
    RequestNonceWord,
    /// A word of the requested device root is p or above.
    RequestRootWord,
    /// The verifier id is empty or longer than `VERIFIER_MAX`.
    RequestVerifierLength,
    /// A verifier id byte is not printable ASCII without space.
    RequestVerifierByte,
    /// The transcript is longer than `TRANSCRIPT_MAX`.
    TranscriptSize,
    /// The transcript is not UTF-8 text.
    TranscriptText,
    /// `Registry::recompute` refused the transcript.
    Transcript(RegistryError),
    /// The transcript's root is not the device root the request accepts.
    RootNotRequested,
    /// The TPM gave no endorsement key.
    NoEk,
    /// An EK answer is not a name and a TPM2B_PUBLIC its size covers.
    EkAnswer,
    /// No endorsement key the TPM gave is enrolled in the transcript.
    NotEnrolled,
    /// The commitment enrolled for this EK is not `commit(secret)`.
    CommitmentMismatch,
    /// A word of the device secret is p or above.
    SecretWord,
    /// The `MkBootSlots` record does not parse, or a path is cut short.
    Slots,
    /// The scope of this verifier id and window cannot be derived.
    Scope,
    /// The proof is longer than the output file holds.
    OutputSize,
}
