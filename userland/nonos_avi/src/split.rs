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

//! A file too long to read whole. AVI keeps its `idx1` index after `movi`,
//! the frames, so a reader that holds only the head never reaches it: every
//! film longer than the head failed with "not a playable avi". The head
//! still says where `movi` ends, from the list's own size, so the index can
//! be read from there and parsed with the head.

use crate::bytes::{fourcc_at, u32_at};
use crate::error::AviError;
use crate::file::AviFile;
use crate::hdrl::{parse_hdrl, Hdrl};
use crate::index::parse_idx1;

/// Where `movi` lies: the offset of its data (its list type, as `parse`
/// counts it) and the offset just past the list, where the next top-level
/// chunk starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoviSpan {
    pub data_pos: u64,
    pub end: u64,
}

/// The `hdrl` list and the `movi` span, from top-level chunk headers only,
/// so `movi` need not fit in `head`.
fn walk(head: &[u8]) -> Result<(Option<Hdrl>, Option<MoviSpan>), AviError> {
    if head.len() < 12 {
        return Err(AviError::Truncated);
    }
    if &head[0..4] != b"RIFF" {
        return Err(AviError::NotRiff);
    }
    if &head[8..12] != b"AVI " {
        return Err(AviError::NotAvi);
    }
    let mut hdrl = None;
    let mut pos = 12usize;
    while pos.checked_add(12).is_some_and(|e| e <= head.len()) {
        let id = fourcc_at(head, pos)?;
        let size = u32_at(head, pos + 4)? as usize;
        let end =
            pos.checked_add(8).and_then(|s| s.checked_add(size)).ok_or(AviError::Truncated)?;
        if id == *b"LIST" && size >= 4 {
            let kind = fourcc_at(head, pos + 8)?;
            if kind == *b"hdrl" {
                let data = head.get(pos + 12..end).ok_or(AviError::Truncated)?;
                hdrl = Some(parse_hdrl(data)?);
            } else if kind == *b"movi" {
                let span = MoviSpan { data_pos: (pos + 8) as u64, end: (end + (end & 1)) as u64 };
                return Ok((hdrl, Some(span)));
            }
        }
        pos = end.checked_add(end & 1).ok_or(AviError::Truncated)?;
    }
    Ok((hdrl, None))
}

/// Where `movi` lies in a file whose `head` holds its headers.
pub fn movi_span(head: &[u8]) -> Result<MoviSpan, AviError> {
    walk(head)?.1.ok_or(AviError::NoFrames)
}

impl AviFile {
    /// Parse from the file's `head`, which must hold `hdrl` and the first
    /// frame of `movi`, and the data of its `idx1` chunk, read from past
    /// `movi`. Frame offsets are the file's own, as `parse` gives them.
    pub fn parse_parts(head: &[u8], idx1: &[u8]) -> Result<AviFile, AviError> {
        let (hdrl, movi) = walk(head)?;
        let hdrl = hdrl.ok_or(AviError::MissingHeader)?;
        let movi = movi.ok_or(AviError::NoFrames)?;
        let index = parse_idx1(head, movi.data_pos, idx1, hdrl.stream)?;
        Ok(AviFile { header: hdrl.header, video: hdrl.video, index })
    }
}
