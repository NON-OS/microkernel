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

//! The running SHA-256 state, for documents too large to hand to the kernel.

pub const BLOCK_BYTES: usize = 64;
pub const DIGEST_BYTES: usize = 32;

/*
 * Why this exists when the kernel already offers SHA-256.
 *
 * `crypto_hash` takes the whole input in one syscall and refuses anything over a
 * megabyte, because it allocates a kernel buffer of that size and copies the
 * caller's bytes into it. A microdescriptor consensus signs about 1.7 MB, so every
 * consensus this client ever fetched failed its digest with errno 22 and was
 * discarded as unverifiable. The transport was never at fault; the document was
 * simply larger than the syscall would carry.
 *
 * Absorbing it here in 64 byte blocks removes both halves of that problem: no cap,
 * and no megabyte allocation inside the kernel for a hash that belongs to a capsule.
 * The kernel keeps the primitive for the small inputs that fit it.
 */
#[derive(Clone)]
pub struct Sha256 {
    pub(super) state: [u32; 8],
    pub(super) block: [u8; BLOCK_BYTES],
    pub(super) buffered: usize,
    pub(super) total: u64,
}

impl Sha256 {
    /// The initial hash value of FIPS 180-4 section 5.3.3: the first thirty two
    /// bits of the fractional parts of the square roots of the first eight primes.
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09_e667,
                0xbb67_ae85,
                0x3c6e_f372,
                0xa54f_f53a,
                0x510e_527f,
                0x9b05_688c,
                0x1f83_d9ab,
                0x5be0_cd19,
            ],
            block: [0u8; BLOCK_BYTES],
            buffered: 0,
            total: 0,
        }
    }
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}
