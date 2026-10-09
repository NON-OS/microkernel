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

//! Writing a key header sector, and sealing the volume key into one.

use super::key_header::{Keyed, Sealed, AAD_END};
use crate::crypto::chacha20poly1305::{aead_decrypt_in_place, aead_encrypt_in_place};
use crate::crypto::constant_time::secure_zero;

/// The sector that says `keyed`, laid out as `key_header` describes.
pub(super) fn encode_key_header(keyed: &Keyed) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDK1");
    match keyed {
        Keyed::Tpm => s[8] = 1,
        Keyed::Passphrase(k) => {
            s[8] = 2;
            s[12..16].copy_from_slice(&k.params.m_kib.to_le_bytes());
            s[16..20].copy_from_slice(&k.params.t.to_le_bytes());
            s[20..24].copy_from_slice(&k.params.p.to_le_bytes());
            s[24..56].copy_from_slice(&k.salt);
            s[56..68].copy_from_slice(&k.nonce);
            s[68..116].copy_from_slice(&k.key_and_tag);
        }
    }
    s
}

fn aad(sealed: &Sealed) -> [u8; AAD_END] {
    let s = encode_key_header(&Keyed::Passphrase(*sealed));
    s[..AAD_END].try_into().unwrap_or([0; AAD_END])
}

/// Seal `key` under `kek` into `sealed`, bound to its parameters and salt.
/// False only if the AEAD refused the buffer, which holds key and tag.
pub(super) fn seal_key(kek: &[u8; 32], sealed: &mut Sealed, key: &[u8; 32]) -> bool {
    let mut buf = [0u8; 48];
    buf[..32].copy_from_slice(key);
    let done = aead_encrypt_in_place(kek, &sealed.nonce, &aad(sealed), &mut buf, 32) == Ok(48);
    sealed.key_and_tag = buf;
    done
}

/// The volume key `sealed` holds, or `None` when `kek` does not open it.
pub(super) fn unseal_key(kek: &[u8; 32], sealed: &Sealed) -> Option<[u8; 32]> {
    let mut buf = sealed.key_and_tag;
    let opened = aead_decrypt_in_place(kek, &sealed.nonce, &aad(sealed), &mut buf, 48);
    let key = match opened {
        Ok(32) => buf[..32].try_into().ok(),
        _ => None,
    };
    secure_zero(&mut buf);
    key
}
