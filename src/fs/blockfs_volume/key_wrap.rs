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

//! Way 3 of the key header: the volume key wrapped under a key the TPM gives
//! only a kernel the release approved (key_pick.rs). The header's bytes 0..56,
//! the way byte with them, are the associated data, so a header relabelled
//! as another way, or changed anywhere in them, does not unwrap.

use super::key_header::{Keyed, Sealed, AAD_END};
use super::key_seal::encode_key_header;
use crate::crypto::chacha20poly1305::{aead_decrypt_in_place, aead_encrypt_in_place};
use crate::crypto::constant_time::secure_zero;
use crate::crypto::util::argon2::Params;

fn aad(wrapped: &Sealed) -> [u8; AAD_END] {
    let s = encode_key_header(&Keyed::Approved(*wrapped));
    s[..AAD_END].try_into().unwrap_or([0; AAD_END])
}

/// `key` wrapped under `kek` with `nonce`, which must be fresh; None only if
/// the AEAD refused the buffer.
pub(super) fn wrap(kek: &[u8; 32], key: &[u8; 32], nonce: [u8; 12]) -> Option<Sealed> {
    let none = Params { m_kib: 0, t: 0, p: 0 };
    let mut w = Sealed { params: none, salt: [0; 32], nonce, key_and_tag: [0; 48] };
    let mut buf = [0u8; 48];
    buf[..32].copy_from_slice(key);
    let done = aead_encrypt_in_place(kek, &w.nonce, &aad(&w), &mut buf, 32) == Ok(48);
    w.key_and_tag = buf;
    done.then_some(w)
}

/// The volume key `wrapped` holds, or None when `kek` does not open it.
pub(super) fn unwrap(kek: &[u8; 32], wrapped: &Sealed) -> Option<[u8; 32]> {
    let mut buf = wrapped.key_and_tag;
    let opened = aead_decrypt_in_place(kek, &wrapped.nonce, &aad(wrapped), &mut buf, 48);
    let key = match opened {
        Ok(32) => buf[..32].try_into().ok(),
        _ => None,
    };
    secure_zero(&mut buf);
    key
}
