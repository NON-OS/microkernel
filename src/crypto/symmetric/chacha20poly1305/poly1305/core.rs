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

//! Key setup and the block loop, after poly1305-donna-64: three limbs,
//! products in u128, no branch or index that depends on key or data.

use super::types::Poly1305;

pub(super) const M44: u64 = 0xfff_ffff_ffff;
pub(super) const M42: u64 = 0x3ff_ffff_ffff;
/// The 2^128 bit every full 16-byte block carries, as seen from limb 2.
pub(super) const HIBIT: u64 = 1 << 40;

#[inline(always)]
fn mul(a: u64, b: u64) -> u128 {
    a as u128 * b as u128
}

impl Poly1305 {
    pub(crate) fn new(key: &[u8; 32]) -> Self {
        let halves = key.as_chunks::<16>().0;
        let t = u128::from_le_bytes(halves[0]);
        let (t0, t1) = (t as u64, (t >> 64) as u64);
        let r0 = t0 & 0xffc_0fff_ffff;
        let r1 = ((t0 >> 44) | (t1 << 20)) & 0xfff_ffc0_ffff;
        let r2 = (t1 >> 24) & 0x00f_ffff_fc0f;
        let p = u128::from_le_bytes(halves[1]);
        let pad = [p as u64, (p >> 64) as u64];
        Self { h: [0; 3], r: [r0, r1, r2], pad, buffer: [0; 16], buffer_len: 0 }
    }

    /// Absorbs each 16-byte block of `msg`, `hibit` set above its top byte.
    pub(super) fn blocks(&mut self, msg: &[[u8; 16]], hibit: u64) {
        let [r0, r1, r2] = self.r;
        let (s1, s2) = (r1 * 20, r2 * 20);
        let [mut h0, mut h1, mut h2] = self.h;
        for m in msg {
            let t = u128::from_le_bytes(*m);
            let (t0, t1) = (t as u64, (t >> 64) as u64);
            h0 += t0 & M44;
            h1 += ((t0 >> 44) | (t1 << 20)) & M44;
            h2 += ((t1 >> 24) & M42) | hibit;
            let d0 = mul(h0, r0) + mul(h1, s2) + mul(h2, s1);
            let d1 = mul(h0, r1) + mul(h1, r0) + mul(h2, s2) + (d0 >> 44);
            let d2 = mul(h0, r2) + mul(h1, r1) + mul(h2, r0) + (d1 >> 44);
            h0 = (d0 as u64 & M44) + (d2 >> 42) as u64 * 5;
            h1 = (d1 as u64 & M44) + (h0 >> 44);
            h2 = d2 as u64 & M42;
            h0 &= M44;
        }
        self.h = [h0, h1, h2];
    }
}
