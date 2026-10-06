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

//! What a v4 signature signs: the document, the hashed part of the
//! signature packet, then 0x04 0xFF and that part's length.

use nonos_hash::{Sha256, Sha512};

use super::refusal::Refusal;

const MD5: u8 = 1;
const SHA1: u8 = 2;
pub const SHA256: u8 = 8;
pub const SHA512: u8 = 10;

pub enum Digest {
    Sha256([u8; 32]),
    Sha512([u8; 64]),
}

impl Digest {
    pub fn bytes(&self) -> &[u8] {
        match self {
            Digest::Sha256(d) => d,
            Digest::Sha512(d) => d,
        }
    }
}

pub fn digest(hash: u8, data: &[u8], hashed: &[u8]) -> Result<Digest, Refusal> {
    let len = u32::try_from(hashed.len()).map_err(|_| Refusal::Malformed)?;
    let mut trailer = [0x04, 0xFF, 0, 0, 0, 0];
    trailer[2..].copy_from_slice(&len.to_be_bytes());
    match hash {
        SHA256 => {
            let mut h = Sha256::new();
            h.update(data);
            h.update(hashed);
            h.update(&trailer);
            Ok(Digest::Sha256(h.finalize()))
        }
        SHA512 => {
            let mut h = Sha512::new();
            h.update(data);
            h.update(hashed);
            h.update(&trailer);
            Ok(Digest::Sha512(h.finalize()))
        }
        MD5 | SHA1 => Err(Refusal::WeakHash),
        _ => Err(Refusal::UnknownHash),
    }
}
