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

//! The PCRs read and quoted together. The values are read first, then
//! quoted, then the quote's digest is checked against them: a value the
//! document carries is one the TPM signed. A PCR extended between the read
//! and the quote moves the digest; the pair is taken once more, and a
//! second disagreement is refused rather than handed out.

use alloc::vec::Vec;

use super::document::QUOTED_PCRS;
use super::error::AttestDocError;
use super::produce_judge::judge;
use super::quote_shape::RESPONSE_MAX;
use crate::security::tpm::boot_reads::read_pcrs;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::quote::build_quote;
use crate::security::tpm::transact_resending;

const TRIES: usize = 2;

pub(super) struct Quoted {
    pub pcrs: [[u8; 32]; QUOTED_PCRS.len()],
    pub attest: Vec<u8>,
    pub signature: Vec<u8>,
}

pub(super) fn quote_pcrs(ak: u32, qualifying: &[u8]) -> Result<Quoted, AttestDocError> {
    let read = || read_pcrs(&QUOTED_PCRS).map_err(|_| AttestDocError::PcrRead);
    let cmd = build_quote(ak, qualifying, &QUOTED_PCRS);
    // SAFETY: eK@nonos.systems - a quote reads PCR state and signs it. It
    // creates no objects and changes no key material.
    let quote = |buf: &mut [u8]| unsafe { transact_resending(&cmd, buf) };
    quote_with(qualifying, read, |buf| quote(buf).map_err(AttestDocError::Tpm))
}

/// The same over any read and quote: the TPM's, or a test's.
pub(super) fn quote_with(
    qualifying: &[u8],
    mut read: impl FnMut() -> Result<[[u8; 32]; QUOTED_PCRS.len()], AttestDocError>,
    mut quote: impl FnMut(&mut [u8]) -> Result<usize, AttestDocError>,
) -> Result<Quoted, AttestDocError> {
    let mut buf = [0u8; RESPONSE_MAX];
    for _ in 0..TRIES {
        let pcrs = read()?;
        let len = quote(&mut buf)?;
        if let Some(q) = judge(
            qualifying,
            pcrs,
            buf.get(..len).ok_or(AttestDocError::Tpm(TpmError::InvalidResponse))?,
        )? {
            return Ok(q);
        }
    }
    Err(AttestDocError::PcrsMoved)
}
