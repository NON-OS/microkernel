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

//! The verdict on one quote, given the values read just before it: the
//! quote must answer the qualifying data, cover exactly the document's
//! selection in the SHA-256 bank, and sign the digest of those values.
//! Bytes in, verdict out, so it is checked against a real TPM's answers.

use alloc::vec::Vec;

use super::document::QUOTED_PCRS;
use super::error::AttestDocError;
use super::produce_quote::Quoted;
use crate::crypto::hash::sha256;
use crate::security::tpm::quote::{check_attest, covered, parse_quote, QuoteError};

const TPM_ALG_SHA256: u16 = 0x000B;

/// `None` when the quote signed other values than `pcrs`: a PCR moved.
pub(super) fn judge(
    qualifying: &[u8],
    pcrs: [[u8; 32]; QUOTED_PCRS.len()],
    response: &[u8],
) -> Result<Option<Quoted>, AttestDocError> {
    let quote = parse_quote(response).map_err(AttestDocError::Quote)?;
    // Checked here, not only by the verifier: a machine that would hand out a
    // quote whose nonce it never confirmed has no idea what it just signed.
    check_attest(quote.attest, qualifying).map_err(AttestDocError::Quote)?;
    let c = covered(quote.attest).map_err(AttestDocError::Quote)?;
    let mut select = [0u8; 3];
    QUOTED_PCRS.iter().for_each(|p| select[usize::from(*p / 8)] |= 1 << (p % 8));
    if c.hash != TPM_ALG_SHA256 || c.select != select {
        return Err(AttestDocError::Quote(QuoteError::Selection));
    }
    if c.digest != sha256(&pcrs.concat()) {
        return Ok(None);
    }
    let (attest, signature) = (Vec::from(quote.attest), Vec::from(quote.signature));
    Ok(Some(Quoted { pcrs, attest, signature }))
}
