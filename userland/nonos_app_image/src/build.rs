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

//! Writing an image: the one layout `Head::parse` accepts. The pack tool
//! writes the same bytes (tools/nonos_linux_pack/image.py); this one is the
//! reference the tests hold it to.

use alloc::vec::Vec;

use crate::format::{HEADER_LEN, IMAGE_MAGIC, MAX_IMAGE, MAX_MEMBERS, MAX_RAW, MEMBER_LEN, VERSION};
use crate::refused::Refused;

/// How a member's tar is stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Codec {
    Tar,
    Xz,
}

/// One member as it goes in: its name, its tar, and the bytes stored for
/// it, the tar itself or its xz stream. The listed package is the first.
pub struct Part<'a> {
    pub name: &'a [u8],
    pub codec: Codec,
    pub tar: &'a [u8],
    pub stored: &'a [u8],
}

pub fn build(parts: &[Part]) -> Result<Vec<u8>, Refused> {
    let names_len: usize = parts.iter().map(|p| p.name.len()).sum();
    let data_at = HEADER_LEN + parts.len() * MEMBER_LEN + names_len;
    let total = parts.iter().fold(data_at as u64, |t, p| t + p.stored.len() as u64);
    let unique = parts.iter().enumerate().all(|(i, p)| parts[..i].iter().all(|q| q.name != p.name));
    let shaped = |p: &Part| {
        crate::name::valid(p.name)
            && !p.tar.is_empty()
            && p.tar.len() % 512 == 0
            && p.tar.len() as u64 <= MAX_RAW
            && !p.stored.is_empty()
            && (p.codec != Codec::Tar || p.stored == p.tar)
    };
    let fits = (1..=MAX_MEMBERS).contains(&parts.len()) && total <= MAX_IMAGE;
    if !fits || !unique || !parts.iter().all(shaped) {
        return Err(Refused::Build);
    }
    let mut out = Vec::with_capacity(total as usize);
    out.extend_from_slice(&IMAGE_MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(&(parts.len() as u16).to_le_bytes());
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(&(names_len as u32).to_le_bytes());
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(&total.to_le_bytes());
    out.resize(HEADER_LEN, 0);
    for (i, p) in parts.iter().enumerate() {
        out.extend_from_slice(&(p.stored.len() as u64).to_le_bytes());
        out.extend_from_slice(&(p.tar.len() as u64).to_le_bytes());
        out.extend_from_slice(&(p.name.len() as u16).to_le_bytes());
        out.push(match p.codec {
            Codec::Tar => 0,
            Codec::Xz => 1,
        });
        out.push(u8::from(i == 0));
        out.extend_from_slice(&[0; 12]);
        out.extend_from_slice(blake3::hash(p.tar).as_bytes());
    }
    for p in parts {
        out.extend_from_slice(p.name);
    }
    for p in parts {
        out.extend_from_slice(p.stored);
    }
    Ok(out)
}
