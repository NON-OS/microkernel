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

//! Writing an image's chunk list. Its BLAKE3 is the image's measurement.

use alloc::vec::Vec;

use crate::format::{CHUNK_MAX, CHUNK_MIN, LIST_HEAD, LIST_MAGIC, MAX_IMAGE, VERSION};
use crate::refused::Refused;

pub fn chunk_list(image: &[u8], chunk: u32) -> Result<Vec<u8>, Refused> {
    let sized = chunk.is_power_of_two() && (CHUNK_MIN..=CHUNK_MAX).contains(&chunk);
    if !sized || image.is_empty() || image.len() as u64 > MAX_IMAGE {
        return Err(Refused::Build);
    }
    let mut out = Vec::with_capacity(LIST_HEAD + image.len().div_ceil(chunk as usize) * 32);
    out.extend_from_slice(&LIST_MAGIC);
    out.extend_from_slice(&u32::from(VERSION).to_le_bytes());
    out.extend_from_slice(&chunk.to_le_bytes());
    out.extend_from_slice(&(image.len() as u64).to_le_bytes());
    out.resize(LIST_HEAD, 0);
    for c in image.chunks(chunk as usize) {
        out.extend_from_slice(blake3::hash(c).as_bytes());
    }
    Ok(out)
}
