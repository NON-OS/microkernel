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

//! One member of an image: a package's files as one tar, where it lies in
//! the image, how it is stored, and the digest of its tar.

use core::ops::Range;

use crate::build::Codec;
use crate::format::{u16_at, u64_at, zero, DIGEST_LEN, MAX_RAW, NAME_MAX};
use crate::refused::Refused;

/// The package the listing sells, or one under it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Listed,
    Need,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member<'a> {
    pub name: &'a [u8],
    pub role: Role,
    pub codec: Codec,
    /// Where its stored bytes lie in the image.
    pub at: Range<u64>,
    /// Its tar's length once decompressed.
    pub raw: u64,
    pub digest: [u8; DIGEST_LEN],
}

impl Member<'_> {
    /// Whether `tar`, this member decompressed, is the tar the table names.
    pub fn holds(&self, tar: &[u8]) -> bool {
        tar.len() as u64 == self.raw && *blake3::hash(tar).as_bytes() == self.digest
    }
}

/// The table row `row`, before its name and offset are known: the stored
/// and raw lengths, the name's length, the codec, the role and the digest.
pub struct Row {
    pub stored: u64,
    pub raw: u64,
    pub name_len: usize,
    pub codec: Codec,
    pub role: Role,
    pub digest: [u8; DIGEST_LEN],
}

pub fn row(r: &[u8]) -> Result<Row, Refused> {
    if !zero(r, 20, 32) {
        return Err(Refused::Magic);
    }
    let (stored, raw, name_len) = (u64_at(r, 0), u64_at(r, 8), u16_at(r, 16) as usize);
    let codec = match r[18] {
        0 => Codec::Tar,
        1 => Codec::Xz,
        _ => return Err(Refused::Member),
    };
    let role = match r[19] {
        0 => Role::Need,
        1 => Role::Listed,
        _ => return Err(Refused::Member),
    };
    if raw > MAX_RAW {
        return Err(Refused::TooLarge);
    }
    // A tar is a whole number of 512-byte blocks, and a stored one is itself.
    let shaped = raw > 0 && raw % 512 == 0 && stored > 0 && (codec != Codec::Tar || stored == raw);
    if !shaped || name_len == 0 || name_len > NAME_MAX {
        return Err(Refused::Layout);
    }
    let mut digest = [0u8; DIGEST_LEN];
    digest.copy_from_slice(&r[32..64]);
    Ok(Row { stored, raw, name_len, codec, role, digest })
}
