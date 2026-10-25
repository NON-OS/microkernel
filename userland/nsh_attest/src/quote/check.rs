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

//! Step 3 of the client's checks, over any PCR list: the structure is a
//! TPM quote, it carries the expected `extraData`, the AK signed it, it
//! selects exactly `pcrs` in the SHA-256 bank, and its digest is over
//! `values`. Any one of these alone is not enough: the signature without
//! the nonce accepts a recording, and the digest without the selection
//! accepts a quote over some other set of PCRs.

use nonos_hash::sha256;

use super::{bitmap, pcr_digest, Attest, EcdsaSig, SELECT_BYTES, TPM_ALG_SHA256};
use crate::consts::AK_LEN;
use crate::Refusal;

/// A SHA-256 name: the algorithm, then 32 bytes.
const SIGNER_LEN: usize = 2 + 32;

pub fn check_quote<'a>(
    attest: &'a [u8],
    signature: &[u8],
    ak_public: &[u8; AK_LEN],
    extra: &[u8; 32],
    pcrs: &[u8],
    values: &[&[u8; 32]],
) -> Result<Attest<'a>, Refusal> {
    let a = Attest::parse(attest)?;
    if a.extra != extra {
        return Err(Refusal::Binding);
    }
    if a.signer.len() != SIGNER_LEN || a.signer[..2] != TPM_ALG_SHA256.to_be_bytes() {
        return Err(Refusal::Signer);
    }
    let sig = EcdsaSig::parse(signature)?;
    if !crate::p256::verify(ak_public, &sha256(attest), &sig.r, &sig.s) {
        return Err(Refusal::Signature);
    }
    let s = a.selection;
    let want: [u8; SELECT_BYTES] = bitmap(pcrs);
    if s.count != 1 || s.hash != TPM_ALG_SHA256 || s.bitmap != want || !listed_once(pcrs) {
        return Err(Refusal::Selection);
    }
    if values.len() != pcrs.len() || a.pcr_digest != pcr_digest(values) {
        return Err(Refusal::PcrDigest);
    }
    Ok(a)
}

/// The caller's list must be ascending with no repeats, the order a single
/// bank's values are digested in, or the values would be read out of turn.
fn listed_once(pcrs: &[u8]) -> bool {
    pcrs.windows(2).all(|w| w[0] < w[1])
}
