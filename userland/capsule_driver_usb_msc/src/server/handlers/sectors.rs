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

//! Moving a span of sectors as the device blocks that cover it. A span that
//! does not start and end on a block boundary is read whole, and for a write
//! the sectors are laid into it before it goes back.

use alloc::vec;

use crate::disk::{read, write, Disk};
use crate::span::Span;
use crate::state::State;

fn span_bytes(disk: &Disk, sp: Span) -> usize {
    sp.blocks as usize * disk.block_len as usize
}

pub(super) fn read_sectors(
    state: &mut State,
    disk: &Disk,
    sp: Span,
    out: &mut [u8],
) -> Result<(), i32> {
    if sp.aligned {
        return read(disk, state, sp.first, out);
    }
    let mut blocks = vec![0u8; span_bytes(disk, sp)];
    read(disk, state, sp.first, &mut blocks)?;
    out.copy_from_slice(&blocks[sp.head..sp.head + out.len()]);
    Ok(())
}

pub(super) fn write_sectors(
    state: &mut State,
    disk: &Disk,
    sp: Span,
    data: &[u8],
) -> Result<(), i32> {
    if sp.aligned {
        return write(disk, state, sp.first, data);
    }
    let mut blocks = vec![0u8; span_bytes(disk, sp)];
    read(disk, state, sp.first, &mut blocks)?;
    blocks[sp.head..sp.head + data.len()].copy_from_slice(data);
    write(disk, state, sp.first, &blocks)
}
