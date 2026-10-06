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

//! The crypto syscalls the kernel forwards to the pool.

use std::cell::RefCell;

use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::protocol::{OP_HMAC_SHA256, OP_SHA256_HASH, OP_X25519_PUBLIC, OP_X25519_SHARED};
use crate::serve::{deliver, input, kernel_call};

thread_local! {
    /* Seeded per thread, so a test that draws keys draws the same ones each run. */
    static RNG: RefCell<ChaCha20Rng> = RefCell::new(ChaCha20Rng::seed_from_u64(8446));
}

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    let out = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    RNG.with(|rng| rng.borrow_mut().fill_bytes(out));
    len as i64
}

pub fn crypto_x25519_public(private: *const u8, out: *mut u8) -> i64 {
    crate::count::note(|c| c.x25519 += 1);
    answer(kernel_call(OP_X25519_PUBLIC, input(private, 32)), out)
}

pub fn crypto_x25519_shared(private: *const u8, public: *const u8, out: *mut u8) -> i64 {
    crate::count::note(|c| c.x25519 += 1);
    let mut body = input(private, 32).to_vec();
    body.extend_from_slice(input(public, 32));
    answer(kernel_call(OP_X25519_SHARED, &body), out)
}

/// Only SHA-256 (algorithm 1) is served; the TLS client asks for no other.
pub fn crypto_hash(algo: u64, data: *const u8, len: usize, out: *mut u8, out_len: usize) -> i64 {
    if algo != 1 || out_len != 32 {
        return -22;
    }
    answer(kernel_call(OP_SHA256_HASH, input(data, len)), out)
}

pub fn crypto_hmac_sha256(
    key: *const u8,
    key_len: usize,
    data: *const u8,
    data_len: usize,
    out: *mut u8,
) -> i64 {
    let mut body = (key_len as u32).to_le_bytes().to_vec();
    body.extend_from_slice(input(key, key_len));
    body.extend_from_slice(input(data, data_len));
    answer(kernel_call(OP_HMAC_SHA256, &body), out)
}

fn answer(result: Result<Vec<u8>, i64>, out: *mut u8) -> i64 {
    result.map_or_else(|status| status, |bytes| deliver(&bytes, out))
}
