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


//! nonos_libc as net.anon's onion code calls it, on the host.
//!
//! The clock and the random source are per thread and set by the test, so a
//! run is the same every time. X25519, SHA-256, HMAC and HKDF are computed
//! here as the crypto pool would compute them. Every trace line net.anon
//! writes is kept for the test to read. IPC reaches nothing: net.tcp and
//! the pool's RSA verify are not part of the onion path.

#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

use std::cell::{Cell, RefCell};

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

thread_local! {
    static NOW_MS: Cell<i64> = const { Cell::new(0) };
    static RANDOM: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Set this thread's clock, in milliseconds.
pub fn set_ms(ms: i64) {
    NOW_MS.with(|n| n.set(ms));
}

/// Seed this thread's random source.
pub fn seed(value: u64) {
    RANDOM.with(|r| r.set(value | 1));
}

/// The trace lines written on this thread since the last call.
pub fn take_log() -> Vec<String> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

fn out<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

pub fn mk_uptime_ms() -> i64 {
    NOW_MS.with(|n| n.get())
}

pub fn mk_time_millis() -> i64 {
    mk_uptime_ms()
}

pub fn mk_yield() -> i64 {
    0
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    let line = String::from_utf8_lossy(bytes(buf, len)).trim_end().to_string();
    LOG.with(|l| l.borrow_mut().push(line));
    len as i64
}

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    for byte in out(buf, len).iter_mut() {
        RANDOM.with(|r| {
            let mut x = r.get();
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            r.set(x);
            *byte = (x >> 24) as u8;
        });
    }
    len as i64
}

pub fn crypto_hash(algo: u64, data: *const u8, len: usize, digest: *mut u8, out_len: usize) -> i64 {
    if algo != 1 || out_len != 32 {
        return -22;
    }
    out(digest, 32).copy_from_slice(&Sha256::digest(bytes(data, len)));
    32
}

pub fn crypto_x25519_public(private: *const u8, public: *mut u8) -> i64 {
    let secret: [u8; 32] = bytes(private, 32).try_into().unwrap();
    let point = x25519_dalek::PublicKey::from(&x25519_dalek::StaticSecret::from(secret));
    out(public, 32).copy_from_slice(point.as_bytes());
    32
}

pub fn crypto_x25519_shared(private: *const u8, public: *const u8, shared: *mut u8) -> i64 {
    let secret: [u8; 32] = bytes(private, 32).try_into().unwrap();
    let peer: [u8; 32] = bytes(public, 32).try_into().unwrap();
    let result = x25519_dalek::StaticSecret::from(secret).diffie_hellman(&x25519_dalek::PublicKey::from(peer));
    out(shared, 32).copy_from_slice(result.as_bytes());
    32
}

pub fn crypto_hmac_sha256(key: *const u8, key_len: usize, data: *const u8, data_len: usize, mac: *mut u8) -> i64 {
    let mut m = <Hmac<Sha256>>::new_from_slice(bytes(key, key_len)).unwrap();
    m.update(bytes(data, data_len));
    out(mac, 32).copy_from_slice(&m.finalize().into_bytes());
    32
}

/// The pool's frame: four little endian u16 widths (out, salt, ikm, info),
/// then salt, ikm and info.
pub fn crypto_hkdf_sha256(frame: *const u8, frame_len: usize, okm: *mut u8, okm_len: usize) -> i64 {
    let f = bytes(frame, frame_len);
    let w = |i: usize| u16::from_le_bytes([f[2 * i], f[2 * i + 1]]) as usize;
    let (salt_len, ikm_len, info_len) = (w(1), w(2), w(3));
    let salt = &f[8..8 + salt_len];
    let ikm = &f[8 + salt_len..8 + salt_len + ikm_len];
    let info = &f[8 + salt_len + ikm_len..8 + salt_len + ikm_len + info_len];
    hkdf::Hkdf::<Sha256>::new(Some(salt), ikm).expand(info, out(okm, okm_len)).unwrap();
    okm_len as i64
}

pub fn mk_ipc_call(_endpoint: u64, _req: *const u8, _req_len: usize, _resp: *mut u8, _resp_len: usize) -> i64 {
    -1
}

pub fn mk_ipc_call_timeout(_e: u64, _r: *const u8, _rl: usize, _o: *mut u8, _ol: usize, _t: u64) -> i64 {
    -1
}

pub fn mk_ipc_reply(_pid: u32, _buf: *const u8, _len: usize) -> i64 {
    0
}

pub fn mk_ipc_recv_from(_e: u64, _buf: *mut u8, _len: usize, _ms: u64, _pid: *mut u32) -> i64 {
    0
}

pub fn mk_service_lookup(_name: *const u8, _len: usize, _port: *mut u32, _pid: *mut u32) -> i64 {
    -1
}

pub fn mk_pid_alive(_pid: u32) -> bool {
    true
}
