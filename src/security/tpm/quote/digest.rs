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

//! What a quote says it covers. After the magic, the type, qualifiedSigner,
//! extraData, clockInfo and firmwareVersion, `TPMS_QUOTE_INFO` names the
//! PCR selection and carries the digest of those PCRs' values (TPM 2.0
//! Part 2, 10.12.1 and 10.12.12). The kernel reads it so that the values it
//! hands out beside a quote are the ones the quote signed.

use super::error::QuoteError;

/// The one selection a quote covers, and the digest of its values.
pub struct Covered<'a> {
    pub hash: u16,
    pub select: &'a [u8],
    pub digest: &'a [u8],
}

/// `attest` has passed `check_attest`, so its magic and type are known. A
/// quote over more than one bank, or with bytes after its digest, is not
/// the one this kernel asked for.
pub fn covered(attest: &[u8]) -> Result<Covered<'_>, QuoteError> {
    let at = tpm2b_end(attest, 6)?;
    let at = tpm2b_end(attest, at)?;
    /* clockInfo: clock u64, resetCount u32, restartCount u32, safe u8; then firmwareVersion u64 */
    let at = at.checked_add(17 + 8).ok_or(QuoteError::Truncated)?;
    if be(attest, at, 4)? != 1 {
        return Err(QuoteError::Selection);
    }
    let hash = be(attest, at + 4, 2)? as u16;
    let size = be(attest, at + 6, 1)? as usize;
    let select = attest.get(at + 7..at + 7 + size).ok_or(QuoteError::Truncated)?;
    let at = at + 7 + size;
    let end = tpm2b_end(attest, at)?;
    if end != attest.len() {
        return Err(QuoteError::Truncated);
    }
    if end - at - 2 != 32 {
        return Err(QuoteError::Selection);
    }
    Ok(Covered { hash, select, digest: &attest[at + 2..end] })
}

/// Where the `TPM2B` at `at` ends.
fn tpm2b_end(b: &[u8], at: usize) -> Result<usize, QuoteError> {
    let n = be(b, at, 2)? as usize;
    let end = at.checked_add(2 + n).ok_or(QuoteError::Truncated)?;
    match end <= b.len() {
        true => Ok(end),
        false => Err(QuoteError::Truncated),
    }
}

/// `n` bytes at `at`, big-endian.
fn be(b: &[u8], at: usize, n: usize) -> Result<u32, QuoteError> {
    let s =
        b.get(at..at.checked_add(n).ok_or(QuoteError::Truncated)?).ok_or(QuoteError::Truncated)?;
    Ok(s.iter().fold(0u32, |v, x| v << 8 | u32::from(*x)))
}
