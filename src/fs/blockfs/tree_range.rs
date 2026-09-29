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

//! Reading any byte range of a file, one data block at a time.

use super::file_consts::DATA_BYTES;
use super::tree_reader::TreeReader;
use super::tree_store::{BlockSource, TreeFault};

/// Copy the file's bytes from `offset` into `out`, stopping at `size`.
/// Returns how many were copied; nothing past `size` is ever read.
pub(crate) fn read_range<S: BlockSource>(
    s: &mut S,
    reader: &mut TreeReader,
    size: u64,
    offset: u64,
    out: &mut [u8],
) -> Result<usize, TreeFault<S::Error>> {
    if offset >= size || out.is_empty() {
        return Ok(0);
    }
    let end = size.min(offset.saturating_add(out.len() as u64));
    let mut pos = offset;
    while pos < end {
        let n = pos / DATA_BYTES as u64;
        let within = (pos % DATA_BYTES as u64) as usize;
        let lba = reader.data_lba(s, n)?;
        let block = s.get(lba).map_err(TreeFault::Store)?;
        let take = ((DATA_BYTES - within) as u64).min(end - pos) as usize;
        let at = (pos - offset) as usize;
        out[at..at + take].copy_from_slice(&block[within..within + take]);
        pos += take as u64;
    }
    Ok((end - offset) as usize)
}
