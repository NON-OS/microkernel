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

use super::error::AttestError;

/// Verify a capsule's proof against the vendor's root.
///
/// A kernel built for STARK attestation accepts only a STARK here, whatever
/// the trailer says it is: letting the trailer choose would let a prover pick
/// the weaker verifier for the root everything shipped is measured under.
///
/// Returns the measurement the proof was checked against, so a caller records
/// what was verified rather than recomputing it and hoping the two agree.
pub(super) fn vendor(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    #[cfg(feature = "nonos-stark-attest")]
    {
        super::stark::verify_against(trailer, elf, granted_caps, root)
    }
    #[cfg(not(feature = "nonos-stark-attest"))]
    {
        super::pedersen_root::verify(trailer, elf, granted_caps, root)
    }
}

/// Verify against a root a human enrolled on this machine. Here the trailer's
/// magic picks the verifier: a local root's leaf is a commitment to a secret
/// only this kernel holds, so the Pedersen proof it mints is sound for it, and
/// minting a STARK locally would cost minutes.
pub(super) fn enrolled(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    /*
     * A build without the STARK verifier has no reader for that magic; the
     * Pedersen parser refuses it as malformed, which is the right answer.
     */
    #[cfg(feature = "nonos-stark-attest")]
    if trailer.starts_with(super::stark::MAGIC) {
        return super::stark::verify_against(trailer, elf, granted_caps, root);
    }
    super::pedersen_root::verify(trailer, elf, granted_caps, root)
}
