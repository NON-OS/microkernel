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


//! Records: the five-byte header and its body, in the clear before
//! ChangeCipherSpec and under the suite's AEAD after it (RFC 5246 6.2,
//! RFC 5288 3 for AES-GCM, RFC 7905 2 for ChaCha20-Poly1305).

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{crypto_decrypt_aad, crypto_encrypt_aad};

use super::constants::{CIPHERTEXT_MAX, PLAINTEXT_MAX, SUITE_CHACHA20, TAG, VERSION};
use super::error::Tls12Error;

/// The kernel AEAD call's algorithm numbers.
const ALGO_CHACHA20_POLY1305: u64 = 0;
const ALGO_AES256_GCM: u64 = 1;
/// AES-GCM's per-record explicit nonce, carried in front of the ciphertext.
const EXPLICIT: usize = 8;

/// One whole record taken off the front of `buf`: its content type and
/// body. `Ok(None)` while the record is still arriving. Any version but
/// TLS 1.2's is refused, except that a server may answer a hello on a
/// TLS 1.0 record (RFC 5246 appendix E.1), so `hello` allows 0x0301 too.
pub fn take(buf: &mut Vec<u8>, hello: bool) -> Result<Option<(u8, Vec<u8>)>, Tls12Error> {
    if buf.len() < 5 {
        return Ok(None);
    }
    let version = u16::from_be_bytes([buf[1], buf[2]]);
    if version != VERSION && !(hello && version == 0x0301) {
        return Err(Tls12Error::Malformed);
    }
    let len = usize::from(u16::from_be_bytes([buf[3], buf[4]]));
    if len == 0 || len > CIPHERTEXT_MAX {
        return Err(Tls12Error::Malformed);
    }
    if buf.len() < 5 + len {
        return Ok(None);
    }
    let kind = buf[0];
    let body = buf[5..5 + len].to_vec();
    buf.drain(..5 + len);
    Ok(Some((kind, body)))
}

/// A plaintext record, split if `body` is longer than one fragment.
pub fn plain(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len() + 5);
    for fragment in body.chunks(PLAINTEXT_MAX) {
        out.push(kind);
        out.extend_from_slice(&VERSION.to_be_bytes());
        out.extend_from_slice(&(fragment.len() as u16).to_be_bytes());
        out.extend_from_slice(fragment);
    }
    out
}

/// One direction's record protection.
pub struct Direction {
    pub suite: u16,
    pub key: [u8; 32],
    /// ChaCha20 uses all twelve bytes, XORed with the sequence number;
    /// AES-GCM uses the first four as the salt in front of the explicit
    /// nonce.
    pub iv: [u8; 12],
    pub seq: u64,
}

impl Drop for Direction {
    fn drop(&mut self) {
        crate::crypto::wipe::wipe(&mut self.key);
        crate::crypto::wipe::wipe(&mut self.iv);
    }
}

impl Direction {
    fn algo(&self) -> u64 {
        if self.suite == SUITE_CHACHA20 {
            ALGO_CHACHA20_POLY1305
        } else {
            ALGO_AES256_GCM
        }
    }

    /// The nonce for sequence number `seq`, and for AES-GCM the explicit
    /// part that goes on the wire.
    fn nonce(&self, explicit: Option<[u8; 8]>) -> [u8; 12] {
        let seq = self.seq.to_be_bytes();
        let mut nonce = self.iv;
        if self.suite == SUITE_CHACHA20 {
            for (n, s) in nonce[4..].iter_mut().zip(seq.iter()) {
                *n ^= s;
            }
        } else {
            nonce[4..].copy_from_slice(&explicit.unwrap_or(seq));
        }
        nonce
    }

    /// The additional data: sequence number, type, version, plaintext length.
    fn aad(&self, kind: u8, len: usize) -> [u8; 13] {
        let mut aad = [0u8; 13];
        aad[..8].copy_from_slice(&self.seq.to_be_bytes());
        aad[8] = kind;
        aad[9..11].copy_from_slice(&VERSION.to_be_bytes());
        aad[11..].copy_from_slice(&(len as u16).to_be_bytes());
        aad
    }

    /// Protect `body`, at most one fragment, as a whole record.
    pub fn seal(&mut self, kind: u8, body: &[u8]) -> Result<Vec<u8>, Tls12Error> {
        if body.len() > PLAINTEXT_MAX {
            return Err(Tls12Error::TooLarge);
        }
        let nonce = self.nonce(None);
        let aad = self.aad(kind, body.len());
        let mut frame = Vec::with_capacity(4 + aad.len() + body.len());
        frame.extend_from_slice(&(aad.len() as u32).to_le_bytes());
        frame.extend_from_slice(&aad);
        frame.extend_from_slice(body);
        let mut sealed = vec![0u8; body.len() + TAG];
        let n = crypto_encrypt_aad(self.algo(), self.key.as_ptr(), nonce.as_ptr(), frame.as_ptr(), frame.len(), sealed.as_mut_ptr());
        crate::crypto::wipe::wipe(&mut frame);
        if n != sealed.len() as i64 {
            return Err(Tls12Error::Crypto);
        }
        let explicit = if self.suite == SUITE_CHACHA20 { 0 } else { EXPLICIT };
        let len = explicit + sealed.len();
        let mut out = Vec::with_capacity(5 + len);
        out.push(kind);
        out.extend_from_slice(&VERSION.to_be_bytes());
        out.extend_from_slice(&(len as u16).to_be_bytes());
        if explicit != 0 {
            out.extend_from_slice(&self.seq.to_be_bytes());
        }
        out.extend_from_slice(&sealed);
        self.seq = self.seq.checked_add(1).ok_or(Tls12Error::Crypto)?;
        Ok(out)
    }

    /// Open the body of a protected record of type `kind`. A body that does
    /// not authenticate, or is shorter than its nonce and tag, is refused.
    pub fn open(&mut self, kind: u8, body: &[u8]) -> Result<Vec<u8>, Tls12Error> {
        let (explicit, sealed) = if self.suite == SUITE_CHACHA20 {
            (None, body)
        } else {
            if body.len() < EXPLICIT {
                return Err(Tls12Error::Malformed);
            }
            let mut e = [0u8; 8];
            e.copy_from_slice(&body[..EXPLICIT]);
            (Some(e), &body[EXPLICIT..])
        };
        if sealed.len() < TAG || sealed.len() - TAG > PLAINTEXT_MAX {
            return Err(Tls12Error::Malformed);
        }
        let len = sealed.len() - TAG;
        let nonce = self.nonce(explicit);
        let aad = self.aad(kind, len);
        let mut frame = Vec::with_capacity(4 + aad.len() + sealed.len());
        frame.extend_from_slice(&(aad.len() as u32).to_le_bytes());
        frame.extend_from_slice(&aad);
        frame.extend_from_slice(sealed);
        let mut plain = vec![0u8; len];
        let n = crypto_decrypt_aad(self.algo(), self.key.as_ptr(), nonce.as_ptr(), frame.as_ptr(), frame.len(), plain.as_mut_ptr());
        if n != len as i64 {
            crate::crypto::wipe::wipe(&mut plain);
            return Err(Tls12Error::Crypto);
        }
        self.seq = self.seq.checked_add(1).ok_or(Tls12Error::Crypto)?;
        Ok(plain)
    }
}
