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

//! The variable-length hash H' of RFC 9106, section 3.3.

use super::blake2b::Blake2b;

/// Fill `out` with H'^T of the concatenated `parts`, T being `out.len()`.
pub(super) fn h_prime(out: &mut [u8], parts: &[&[u8]]) {
    let t = out.len();
    let mut first = Blake2b::new(t.min(64));
    first.update(&(t as u32).to_le_bytes());
    for part in parts {
        first.update(part);
    }
    if t <= 64 {
        first.finalize(out);
        return;
    }
    /*
     * Longer outputs chain 64-byte digests: the first 32 bytes of each
     * are kept, and the last digest is as long as what is left over.
     */
    let mut v = [0u8; 64];
    first.finalize(&mut v);
    let r = t.div_ceil(32) - 2;
    out[..32].copy_from_slice(&v[..32]);
    for i in 1..r {
        let mut next = Blake2b::new(64);
        next.update(&v);
        next.finalize(&mut v);
        out[i * 32..i * 32 + 32].copy_from_slice(&v[..32]);
    }
    let rest = t - 32 * r;
    let mut last = Blake2b::new(rest);
    last.update(&v);
    last.finalize(&mut out[32 * r..]);
    crate::crypto::constant_time::secure_zero(&mut v);
}
