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

use super::consts::{TPM_HEADER_LEN, TPM_RC_SUCCESS, TPM_ST_SESSIONS};
use super::error::QuoteError;

/// The two halves a verifier needs: the structure the TPM signed, and the
/// signature over it.
///
/// `attest` is kept as raw bytes rather than parsed into fields. A verifier
/// must hash exactly what was signed, and re-serialising a parsed structure is
/// how a byte of padding or an unhandled field silently changes the digest.
pub struct QuoteResult<'a> {
    pub attest: &'a [u8],
    pub signature: &'a [u8],
}

/// Split a `TPM2_Quote` response into the signed structure and its signature.
///
/// The code is checked first: a failed command still has a well-formed
/// header. Quote is sent with a session, so the parameters sit behind a 4-byte
/// `parameterSize` and the session's answer follows them; reading from the
/// header on would take the size for the attest length.
pub fn parse_quote(resp: &[u8]) -> Result<QuoteResult<'_>, QuoteError> {
    let head = resp.get(..TPM_HEADER_LEN).ok_or(QuoteError::Truncated)?;
    let code = u32::from_be_bytes([head[6], head[7], head[8], head[9]]);
    if code != TPM_RC_SUCCESS {
        return Err(QuoteError::Tpm(code));
    }
    let size = u32::from_be_bytes([head[2], head[3], head[4], head[5]]) as usize;
    let body = resp.get(TPM_HEADER_LEN..size).ok_or(QuoteError::Truncated)?;
    if u16::from_be_bytes([head[0], head[1]]) != TPM_ST_SESSIONS {
        return Err(QuoteError::NotAQuote);
    }
    let n = body.get(..4).ok_or(QuoteError::Truncated)?;
    let n = u32::from_be_bytes([n[0], n[1], n[2], n[3]]) as usize;
    let params = body.get(4..4usize.saturating_add(n)).ok_or(QuoteError::Truncated)?;
    /* TPM2B_ATTEST, then the TPMT_SIGNATURE whole: its shape is the key's. */
    let len = params.get(..2).ok_or(QuoteError::Truncated)?;
    let end = 2 + u16::from_be_bytes([len[0], len[1]]) as usize;
    let attest = params.get(2..end).ok_or(QuoteError::Truncated)?;
    let signature = params.get(end..).ok_or(QuoteError::Truncated)?;
    if attest.is_empty() || signature.is_empty() {
        return Err(QuoteError::Truncated);
    }
    Ok(QuoteResult { attest, signature })
}
