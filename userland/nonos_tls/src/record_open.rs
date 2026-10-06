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

use alloc::vec::Vec;

use super::constants::SUITE_AES128_GCM_SHA256;

/// RFC 8446 5.2: a protected record's body is at most 2^14 + 256 bytes.
const CIPHERTEXT_MAX: usize = (1 << 14) + 256;
/// And what it decrypts to, the content type included, at most 2^14 + 1.
const PLAINTEXT_MAX: usize = (1 << 14) + 1;

/*
 * A record over either bound is record_overflow (RFC 8446 5.2, 5.4) and is
 * refused like one that does not decrypt; a peer that sealed more had all
 * of it taken.
 */
pub fn open(suite: u16, key: &[u8; 32], iv: &[u8; 12], seq: u64, record: &[u8]) -> Option<Vec<u8>> {
    if record.len() < 22 || record.first() != Some(&23) {
        return None;
    }
    let len = super::read::u16_at(record, 3)? as usize;
    if len + 5 != record.len() || len <= 16 || len > CIPHERTEXT_MAX {
        return None;
    }
    let nonce = super::nonce::nonce(iv, seq);
    let plain = if suite == SUITE_AES128_GCM_SHA256 {
        let mut k = [0u8; 16];
        k.copy_from_slice(&key[..16]);
        super::aes_gcm::open(&k, &nonce, &record[..5], &record[5..])?
    } else {
        super::chacha_record::open(key, &nonce, &record[..5], &record[5..])?
    };
    (plain.len() <= PLAINTEXT_MAX).then_some(plain)
}
