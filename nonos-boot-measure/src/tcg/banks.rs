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

use super::consts::{MAX_ALGS, MAX_DIGEST, SPEC_ID_SIGNATURE, TPM_ALG_SHA256};
use super::error::LogError;
use super::read::{Cursor, Stop};

/// The banks the log declares, `(algorithm, digest size)`, in header order.
#[derive(Clone, Copy, Debug)]
pub struct Banks {
    algs: [(u16, u16); MAX_ALGS],
    n: usize,
}

impl Banks {
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// The digest size of a declared bank.
    pub fn size_of(&self, alg: u16) -> Option<usize> {
        self.algs.get(..self.n)?.iter().find(|a| a.0 == alg).map(|a| a.1 as usize)
    }
}

/*
 * The Spec ID body: the signature, platform class, three version bytes and
 * uintnSize, the bank table, then the vendor bytes. A bank is declared once,
 * with a digest no longer than SHA-512's, and SHA-256 must be among them.
 */
pub(super) fn parse_banks(body: &[u8]) -> Result<Banks, Stop> {
    let mut b = Cursor::new(body);
    if b.take(16)? != SPEC_ID_SIGNATURE {
        return Err(Stop::Bad(LogError::NotCryptoAgile));
    }
    b.take(8)?;
    let n = b.u32()? as usize;
    if n > MAX_ALGS {
        return Err(Stop::Bad(LogError::TooManyBanks));
    }
    let mut out = Banks { algs: [(0, 0); MAX_ALGS], n: 0 };
    for i in 0..n {
        let (alg, size) = (b.u16()?, b.u16()?);
        if size as usize > MAX_DIGEST || out.size_of(alg).is_some() {
            return Err(Stop::Bad(LogError::UnknownBank));
        }
        out.algs[i] = (alg, size);
        out.n = i + 1;
    }
    let vendor = b.u8()? as usize;
    b.take(vendor)?;
    match out.size_of(TPM_ALG_SHA256) {
        Some(32) => Ok(out),
        _ => Err(Stop::Bad(LogError::NoSha256Bank)),
    }
}
