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

//! SHA-1 for a TPM's SHA-1 PCR bank, and for nothing else. A TPM that keeps
//! a SHA-1 bank is extended with SHA-1 digests, so an event the kernel
//! measures carries one there. Nothing rests on SHA-1's strength: a verifier
//! replays and trusts the SHA-256 bank alone. So it is not deprecated, and
//! it is crate-private so that no other caller takes it; anything else
//! hashes with SHA-256 or SHA-3. The general `sha1` (sha1.rs) is this same
//! digest under its deprecation.

/// The SHA-1 digest of `data`, for a TPM's SHA-1 bank alone.
pub(crate) fn tpm_bank_sha1(data: &[u8]) -> [u8; 20] {
    let mut h = [0x67452301u32, 0xEFCDAB89u32, 0x98BADCFEu32, 0x10325476u32, 0xC3D2E1F0u32];

    let mut message = data.to_vec();
    let bit_len = (message.len() as u64) * 8;
    message.push(0x80);
    while (message.len() % 64) != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in message.chunks_exact(64) {
        let mut w = [0u32; 80];
        for (word, bytes) in w.iter_mut().zip(chunk.chunks_exact(4)) {
            *word = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }

        let [mut a, mut b, mut c, mut d, mut e] = h;
        for (i, &wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp =
                a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        for (word, add) in h.iter_mut().zip([a, b, c, d, e]) {
            *word = word.wrapping_add(add);
        }
    }

    let mut result = [0u8; 20];
    for (out, word) in result.chunks_exact_mut(4).zip(h) {
        out.copy_from_slice(&word.to_be_bytes());
    }
    result
}
