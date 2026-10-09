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

//! The key header: one plain sector, right after the disk plan, saying how
//! the volume's key is reached. It lies below `DATA_FLOOR`, so no range the
//! plan names can cover it. A disk without it holds a TPM-keyed volume.
//!
//!   bytes  0..8    magic "NONOSDK1"
//!          8       1: TPM machine key; 2: passphrase
//!          9..12   zero
//!         12..24   passphrase: Argon2id KiB, passes, lanes (u32 each)
//!         24..56   passphrase: the salt
//!         56..68   passphrase: the nonce the volume key is sealed under
//!         68..116  passphrase: the volume key and its tag
//!
//! The volume key is random; the Argon2id output only seals it, with bytes
//! 0..56 as the associated data, so a wrong passphrase or a changed
//! parameter fails the tag before any volume sector is read.

use super::plan_types::PLAN_LBA;
use crate::crypto::util::argon2::Params;

pub(super) const KEY_LBA: u64 = PLAN_LBA + 1;
const MAGIC: [u8; 8] = *b"NONOSDK1";
pub(super) const AAD_END: usize = 56;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Sealed {
    pub(super) params: Params,
    pub(super) salt: [u8; 32],
    pub(super) nonce: [u8; 12],
    pub(super) key_and_tag: [u8; 48],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Keyed {
    Tpm,
    Passphrase(Sealed),
}

fn word(s: &[u8; 512], at: usize) -> u32 {
    u32::from_le_bytes([s[at], s[at + 1], s[at + 2], s[at + 3]])
}

/// What `sector` says, `None` when it is no key header at all, or `Err`
/// naming the way byte of a header this kernel does not know.
pub(super) fn parse_key_header(s: &[u8; 512]) -> Result<Option<Keyed>, u8> {
    if s[..8] != MAGIC {
        return Ok(None);
    }
    match s[8] {
        1 => Ok(Some(Keyed::Tpm)),
        2 => Ok(Some(Keyed::Passphrase(Sealed {
            params: Params { m_kib: word(s, 12), t: word(s, 16), p: word(s, 20) },
            salt: s[24..56].try_into().unwrap_or([0; 32]),
            nonce: s[56..68].try_into().unwrap_or([0; 12]),
            key_and_tag: s[68..116].try_into().unwrap_or([0; 48]),
        }))),
        other => Err(other),
    }
}
