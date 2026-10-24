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

//! ECDSA verification over P-256 (FIPS 186-5, 6.4.2): r and s in 1..n-1,
//! the key a point on the curve, and the x of u1*G + u2*Q equal to r mod n.

use super::{AffinePoint, Scalar};

/// Whether (r, s) signs `digest` under the uncompressed point `xy`.
pub fn verify(xy: &[u8; 64], digest: &[u8; 32], r: &[u8; 32], s: &[u8; 32]) -> bool {
    let mut sec1 = [0u8; 65];
    sec1[0] = 0x04;
    sec1[1..].copy_from_slice(xy);
    let Some(q) = AffinePoint::from_uncompressed(&sec1) else {
        return false;
    };
    let (Some(r), Some(s)) = (Scalar::from_bytes(r), Scalar::from_bytes(s)) else {
        return false;
    };
    if r.is_zero() || s.is_zero() {
        return false;
    }
    let Some(w) = s.invert() else {
        return false;
    };
    let z = Scalar::from_bytes_reduce(digest);
    let (u1, u2) = (z.mul(&w), r.mul(&w));
    let g = AffinePoint::generator().to_projective();
    let p = g.mul(&u1).add(&q.to_projective().mul(&u2)).to_affine();
    if p.infinity {
        return false;
    }
    Scalar::from_bytes_reduce(&p.x.to_bytes()).ct_eq(&r)
}
