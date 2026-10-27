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

//! Sealing the handover record: ChaCha20-Poly1305 under the handover key,
//! with the header's bytes 0..56 and the record's magic as the associated
//! data. A record moved under another header, or under one changed in any
//! byte way 3 binds, does not unwrap. Way 3's own wrap, bytes 56..116, is
//! not bound, so the record stays good while that wrap is replaced
//! (layout.rs `moved`). Pure.

use super::super::key_header::AAD_END;
use super::layout::{Record, MAGIC};
use crate::crypto::chacha20poly1305::{aead_decrypt_in_place, aead_encrypt_in_place};
use crate::crypto::constant_time::secure_zero;

fn aad(s: &[u8; 512]) -> [u8; AAD_END + 8] {
    let mut a = [0u8; AAD_END + 8];
    a[..AAD_END].copy_from_slice(&s[..AAD_END]);
    a[AAD_END..].copy_from_slice(&MAGIC);
    a
}

/// `key` sealed under `kek` with `nonce`, which must be fresh, for the
/// header `s`; None only if the AEAD refused the buffer.
pub(super) fn wrap(
    kek: &[u8; 32],
    key: &[u8; 32],
    nonce: [u8; 12],
    s: &[u8; 512],
) -> Option<Record> {
    let mut buf = [0u8; 48];
    buf[..32].copy_from_slice(key);
    let done = aead_encrypt_in_place(kek, &nonce, &aad(s), &mut buf, 32) == Ok(48);
    done.then_some(Record { nonce, key_and_tag: buf })
}

/// The volume key `r` holds under the header `s`, or None when `kek` does
/// not open it.
pub(super) fn unwrap(kek: &[u8; 32], r: &Record, s: &[u8; 512]) -> Option<[u8; 32]> {
    let mut buf = r.key_and_tag;
    let opened = aead_decrypt_in_place(kek, &r.nonce, &aad(s), &mut buf, 48);
    let key = match opened {
        Ok(32) => buf[..32].try_into().ok(),
        _ => None,
    };
    secure_zero(&mut buf);
    key
}
