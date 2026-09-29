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

//! A GGUF header checked end to end: magic, version, counts, metadata, every
//! tensor's description, and where every tensor's bytes lie.

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::layout::{align_up, check_layout};
use crate::limits::Limits;
use crate::meta::read_meta;
use crate::source::ReadAt;
use crate::summary::Summary;
use crate::tensor::read_tensor;
use crate::types::TYPE_SLOTS;

const MAGIC: [u8; 4] = *b"GGUF";

/// Check the GGUF file of `len` bytes that `src` reads. Nothing in the file
/// is trusted: each field is bounded by `lim` or by the file's length before
/// it sizes anything.
pub fn parse<S: ReadAt>(src: &mut S, len: u64, lim: &Limits) -> Result<Summary, GgufError> {
    let mut c = Cursor::new(src, len);
    if c.take::<4>()? != MAGIC {
        return Err(GgufError::BadMagic);
    }
    let version = c.u32()?;
    if version != 2 && version != 3 {
        return Err(GgufError::UnsupportedVersion(version));
    }
    let tensors = c.u64()?;
    if tensors > lim.max_tensors {
        return Err(GgufError::TooManyTensors(tensors));
    }
    let keys = c.u64()?;
    if keys > lim.max_keys {
        return Err(GgufError::TooManyKeys(keys));
    }
    let meta = read_meta(&mut c, keys, lim)?;
    let mut infos = Vec::with_capacity(tensors as usize);
    let mut by_type = [0u32; TYPE_SLOTS];
    for t in 0..tensors {
        let info = read_tensor(&mut c, t, lim)?;
        by_type[info.ty as usize] += 1;
        infos.push(info);
    }
    let align = u64::from(meta.alignment);
    let data_start = align_up(c.pos, align).ok_or(GgufError::Truncated { at: c.pos })?;
    let data_bytes = check_layout(&infos, data_start, align, len)?;
    Ok(Summary { version, tensors, keys, meta, data_start, data_bytes, by_type })
}
