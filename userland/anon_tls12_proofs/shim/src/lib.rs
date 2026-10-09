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


//! nonos_libc on the host, for net.anon's TLS 1.2 module.

use std::cell::Cell;
use std::time::Instant;

use aes_gcm::aead::{Aead, KeyInit, Payload};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

thread_local! {
    static RANDOM: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
    static OFFSET: Cell<i64> = const { Cell::new(0) };
    static START: Cell<Option<Instant>> = const { Cell::new(None) };
}

/// Restart the random stream, so a handshake draws the same bytes again.
pub fn seed(value: u64) {
    RANDOM.with(|r| r.set(value | 1));
}

/// Move the clock on, so a replay that has run dry reaches its deadline
/// without waiting for it.
pub fn advance(ms: i64) {
    OFFSET.with(|o| o.set(o.get() + ms));
}

pub fn mk_uptime_ms() -> i64 {
    let start = START.with(|s| {
        if s.get().is_none() {
            s.set(Some(Instant::now()));
        }
        s.get().unwrap()
    });
    start.elapsed().as_millis() as i64 + OFFSET.with(|o| o.get())
}

pub fn mk_yield() -> i64 {
    0
}

fn slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(ptr, len) }
}

fn deliver(bytes: &[u8], out: *mut u8) -> i64 {
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len()) };
    bytes.len() as i64
}

/// xorshift64*: not a cryptographic source, which a replay does not want.
pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    let out = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    for byte in out.iter_mut() {
        let next = RANDOM.with(|r| {
            let mut x = r.get();
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            r.set(x);
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        });
        *byte = (next >> 56) as u8;
    }
    len as i64
}

pub fn crypto_hash(algo: u64, data: *const u8, len: usize, out: *mut u8, out_len: usize) -> i64 {
    if algo != 1 || out_len < 32 {
        return -22;
    }
    deliver(&Sha256::digest(slice(data, len)), out)
}

pub fn crypto_hmac_sha256(key: *const u8, key_len: usize, data: *const u8, data_len: usize, mac: *mut u8) -> i64 {
    let mut m = <Hmac<Sha256> as Mac>::new_from_slice(slice(key, key_len)).expect("any key length");
    m.update(slice(data, data_len));
    deliver(&m.finalize().into_bytes(), mac)
}

/// Not reached by the TLS 1.2 module; present because prf.rs names it.
pub fn crypto_hkdf_sha256(_frame: *const u8, _frame_len: usize, _okm: *mut u8, _okm_len: usize) -> i64 {
    -22
}

/// The frame is `aad_len u32 LE || aad || payload`, as the kernel takes it.
fn split(frame: &[u8]) -> Option<(&[u8], &[u8])> {
    let n = u32::from_le_bytes(frame.get(..4)?.try_into().ok()?) as usize;
    let aad = frame.get(4..4 + n)?;
    Some((aad, &frame[4 + n..]))
}

fn aead(seal: bool, algo: u64, key: *const u8, nonce: *const u8, frame: *const u8, frame_len: usize, out: *mut u8) -> i64 {
    let (key, nonce) = (slice(key, 32), slice(nonce, 12));
    let Some((aad, msg)) = split(slice(frame, frame_len)) else { return -22 };
    let payload = Payload { msg, aad };
    let result = match algo {
        0 => {
            let c = chacha20poly1305::ChaCha20Poly1305::new_from_slice(key).unwrap();
            if seal { c.encrypt(nonce.into(), payload) } else { c.decrypt(nonce.into(), payload) }
        }
        1 => {
            let c = aes_gcm::Aes256Gcm::new_from_slice(key).unwrap();
            if seal { c.encrypt(nonce.into(), payload) } else { c.decrypt(nonce.into(), payload) }
        }
        _ => return -22,
    };
    match result {
        Ok(bytes) => deliver(&bytes, out),
        Err(_) => -74,
    }
}

pub fn crypto_encrypt_aad(algo: u64, key: *const u8, nonce: *const u8, frame: *const u8, frame_len: usize, out: *mut u8) -> i64 {
    aead(true, algo, key, nonce, frame, frame_len, out)
}

pub fn crypto_decrypt_aad(algo: u64, key: *const u8, nonce: *const u8, frame: *const u8, frame_len: usize, out: *mut u8) -> i64 {
    aead(false, algo, key, nonce, frame, frame_len, out)
}
