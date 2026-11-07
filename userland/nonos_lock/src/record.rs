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

//! The verifier record: the Argon2id parameters, the salt, the tag and the
//! wrong entries since the last right one. 62 bytes, little-endian:
//!
//! ```text
//! magic "NLK1" | m_kib u32 | t u8 | p u8 | salt[16] | tag[32] | failures u32
//! ```

use crate::argon2::{Params, MAX_P, MAX_T};
use crate::wipe::secure_zero;

pub const MAGIC: [u8; 4] = *b"NLK1";
pub const SALT_LEN: usize = 16;
pub const TAG_LEN: usize = 32;
pub const RECORD_LEN: usize = 4 + 4 + 1 + 1 + SALT_LEN + TAG_LEN + 4;

/// RFC 9106's second recommended set (section 4): 64 MiB, three passes,
/// four lanes. The kernel's data volume uses the same (`RECOMMENDED`).
pub const PARAMS: Params = Params { m_kib: 64 * 1024, t: 3, p: 4 };

/// The most memory a record may ask for: the keyring sizes its heap for
/// `PARAMS` and no more.
pub const MAX_M_KIB: u32 = PARAMS.m_kib;

pub struct Record {
    pub params: Params,
    pub salt: [u8; SALT_LEN],
    pub tag: [u8; TAG_LEN],
    pub failures: u32,
}

impl Record {
    pub fn encode(&self) -> [u8; RECORD_LEN] {
        let mut out = [0u8; RECORD_LEN];
        out[0..4].copy_from_slice(&MAGIC);
        out[4..8].copy_from_slice(&self.params.m_kib.to_le_bytes());
        out[8] = self.params.t as u8;
        out[9] = self.params.p as u8;
        out[10..26].copy_from_slice(&self.salt);
        out[26..58].copy_from_slice(&self.tag);
        out[58..62].copy_from_slice(&self.failures.to_le_bytes());
        out
    }

    /// The record in `bytes`, or None when it is not one: the wrong length
    /// or magic, or parameters outside what the keyring can run.
    pub fn decode(bytes: &[u8]) -> Option<Record> {
        if bytes.len() != RECORD_LEN || bytes[0..4] != MAGIC {
            return None;
        }
        let m_kib = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let params = Params { m_kib, t: u32::from(bytes[8]), p: u32::from(bytes[9]) };
        if !runnable(params) {
            return None;
        }
        let mut salt = [0u8; SALT_LEN];
        salt.copy_from_slice(&bytes[10..26]);
        let mut tag = [0u8; TAG_LEN];
        tag.copy_from_slice(&bytes[26..58]);
        let failures = u32::from_le_bytes([bytes[58], bytes[59], bytes[60], bytes[61]]);
        Some(Record { params, salt, tag, failures })
    }
}

/// Parameters the keyring can run: within RFC 9106's ranges and the
/// memory it holds for them.
pub fn runnable(p: Params) -> bool {
    (1..=MAX_P).contains(&p.p)
        && (1..=MAX_T).contains(&p.t)
        && p.m_kib >= 8 * p.p
        && p.m_kib <= MAX_M_KIB
}

impl Drop for Record {
    fn drop(&mut self) {
        secure_zero(&mut self.salt);
        secure_zero(&mut self.tag);
    }
}
