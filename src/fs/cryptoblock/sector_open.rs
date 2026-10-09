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

//! Opening one sealed sector into a block the caller holds, with no heap.
//!
//! The same ChaCha20-Poly1305 open `aead_decrypt` performs, fixed to the
//! sector layout (nonce, 484 sealed bytes, tag) and the sector's AAD. The
//! tag is checked over the sealed bytes first; only a sector that passes is
//! deciphered, straight into `out`, so a forged sector writes nothing there.
//! The one-time key and the key stream are wiped before return.

use super::constants::{AAD_PREFIX, NONCE_BYTES, PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use crate::crypto::chacha20poly1305::{chacha20_block, poly1305_mac};
use crate::crypto::constant_time::{ct_eq, secure_zero};

const AAD_BYTES: usize = 24;
/*
 * The Poly1305 input: AAD and sealed bytes, each padded to 16, then the
 * two lengths.
 */
const CT_AT: usize = AAD_BYTES.div_ceil(16) * 16;
const LENS_AT: usize = CT_AT + PLAIN_BLOCK_BYTES.div_ceil(16) * 16;
const MAC_BYTES: usize = LENS_AT + 16;
const TAG_AT: usize = NONCE_BYTES + PLAIN_BLOCK_BYTES;

/// True when the sector sealed at `lba` under `key` opened into `out`;
/// false, with `out` untouched, when its tag does not match.
pub(crate) fn open_sealed(
    key: &[u8; 32],
    lba: u64,
    sector: &[u8; SECTOR_BYTES],
    out: &mut [u8; PLAIN_BLOCK_BYTES],
) -> bool {
    let mut nonce = [0u8; NONCE_BYTES];
    nonce.copy_from_slice(&sector[..NONCE_BYTES]);
    let sealed = &sector[NONCE_BYTES..TAG_AT];
    let mut msg = [0u8; MAC_BYTES];
    msg[..16].copy_from_slice(AAD_PREFIX);
    msg[16..AAD_BYTES].copy_from_slice(&lba.to_le_bytes());
    msg[CT_AT..CT_AT + PLAIN_BLOCK_BYTES].copy_from_slice(sealed);
    msg[LENS_AT..LENS_AT + 8].copy_from_slice(&(AAD_BYTES as u64).to_le_bytes());
    msg[LENS_AT + 8..].copy_from_slice(&(PLAIN_BLOCK_BYTES as u64).to_le_bytes());
    let mut stream = [0u8; 64];
    chacha20_block(key, &nonce, 0, &mut stream);
    let mut otk = [0u8; 32];
    otk.copy_from_slice(&stream[..32]);
    let tag_ok = ct_eq(&poly1305_mac(&msg, &otk), &sector[TAG_AT..]);
    secure_zero(&mut otk);
    if tag_ok {
        for (i, chunk) in sealed.chunks(64).enumerate() {
            chacha20_block(key, &nonce, 1 + i as u32, &mut stream);
            let at = i * 64;
            for (j, b) in chunk.iter().enumerate() {
                out[at + j] = b ^ stream[j];
            }
        }
    }
    secure_zero(&mut stream);
    tag_ok
}
