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

//! Multiprecision integers: a bit count, then the big-endian magnitude. A
//! count that disagrees with the magnitude's top bit is refused.

use alloc::vec::Vec;

use super::refusal::Refusal;

/// Native Ed25519 writes its signature raw; every other algorithm as MPIs.
const ED25519: u8 = 27;

pub fn mpi(d: &[u8]) -> Option<(&[u8], &[u8])> {
    let bits = usize::from(u16::from_be_bytes([*d.first()?, *d.get(1)?]));
    let n = bits.div_ceil(8);
    let v = d.get(2..2 + n)?;
    if let Some(&top) = v.first() {
        if (n - 1) * 8 + (8 - top.leading_zeros() as usize) != bits {
            return None;
        }
    }
    Some((v, &d[2 + n..]))
}

/// A signature's values: one raw 64-byte value for native Ed25519, else up
/// to two MPIs, with nothing after them.
pub fn values(algo: u8, mut rest: &[u8]) -> Result<Vec<&[u8]>, Refusal> {
    let mut out = Vec::new();
    if algo == ED25519 {
        out.push(rest.get(..64).ok_or(Refusal::Malformed)?);
        rest = &rest[64..];
    }
    while !rest.is_empty() && out.len() < 2 {
        let (v, after) = mpi(rest).ok_or(Refusal::Malformed)?;
        out.push(v);
        rest = after;
    }
    rest.is_empty().then_some(out).ok_or(Refusal::Malformed)
}

/// An MPI right-aligned into its fixed width; one too wide is malformed.
pub fn fixed(out: &mut [u8], v: &[u8]) -> Result<(), Refusal> {
    let pad = out.len().checked_sub(v.len()).ok_or(Refusal::Malformed)?;
    out[pad..].copy_from_slice(v);
    Ok(())
}
